use crate::error::BotError;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

/// SQLite에 연결하고 마이그레이션을 실행한다.
/// `db_path`는 `~/...` 형태 허용 (shellexpand 적용).
pub async fn connect(db_path: &str) -> Result<SqlitePool, BotError> {
    let expanded = shellexpand::tilde(db_path).into_owned();

    // 부모 디렉터리 생성
    if let Some(parent) = std::path::Path::new(&expanded).parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(BotError::Io)?;
    }

    let url = format!("sqlite://{}?mode=rwc", expanded);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                sqlx::query("PRAGMA journal_mode=WAL")
                    .execute(&mut *conn)
                    .await?;
                // FULL: 각 커밋마다 WAL을 fsync — 전원 장애 시에도 커밋 손실 없음.
                // NORMAL 대비 약간 느리지만 금융 거래 DB에서 필수.
                sqlx::query("PRAGMA synchronous=FULL")
                    .execute(&mut *conn)
                    .await?;
                // WAL 파일이 1000페이지(약 4MB) 초과 시 자동 체크포인트.
                // 미설정 시 장기 운용 중 WAL 파일이 수백 MB로 누적될 수 있음.
                sqlx::query("PRAGMA wal_autocheckpoint=1000")
                    .execute(&mut *conn)
                    .await?;
                Ok(())
            })
        })
        .connect(&url)
        .await?;

    sqlx::migrate!("src/db/migrations").run(&pool).await?;

    Ok(pool)
}
