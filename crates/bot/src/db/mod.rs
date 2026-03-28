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
                sqlx::query("PRAGMA synchronous=NORMAL")
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
