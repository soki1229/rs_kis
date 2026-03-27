use kis_bot::db::connect;
use tempfile::tempdir;

#[tokio::test]
async fn connect_creates_schema() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db").to_str().unwrap().to_string();

    let pool = connect(&db_path).await.expect("connect should succeed");

    // orders 테이블 존재 확인
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='orders'"
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(row.0, 1, "orders table should exist");

    // positions 테이블 존재 확인
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='positions'"
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(row.0, 1, "positions table should exist");
}

#[tokio::test]
async fn connect_idempotent() {
    // 동일 경로에 두 번 연결해도 마이그레이션이 중복 실행되지 않아야 함
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db").to_str().unwrap().to_string();

    let _pool1 = connect(&db_path).await.unwrap();
    let pool2 = connect(&db_path).await.unwrap();

    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='signal_log'"
    )
    .fetch_one(&pool2)
    .await
    .unwrap();
    assert_eq!(row.0, 1);
}
