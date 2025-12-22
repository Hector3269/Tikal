use std::sync::Arc;
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage, ImageExt};
use tikal::domain::QueryExecutor;
use tikal::infrastructure::config::DatabaseConfig;
use tikal::infrastructure::database::drivers::{MySQLDriver, PostgresDriver, SQLiteDriver};
use tikal::infrastructure::database::executor::SqlxQueryExecutor;

async fn start_postgres_container() -> ContainerAsync<GenericImage> {
    let image = GenericImage::new("postgres", "15")
        .with_env_var("POSTGRES_USER", "admin")
        .with_env_var("POSTGRES_PASSWORD", "secret")
        .with_env_var("POSTGRES_DB", "my_postgres_db");

    image
        .start()
        .await
        .expect("Failed to start PostgreSQL container")
}

async fn start_mysql_container() -> ContainerAsync<GenericImage> {
    let image = GenericImage::new("mysql", "8")
        .with_env_var("MYSQL_ROOT_PASSWORD", "secret")
        .with_env_var("MYSQL_DATABASE", "my_mysql_db")
        .with_env_var("MYSQL_USER", "admin")
        .with_env_var("MYSQL_PASSWORD", "secret");

    image
        .start()
        .await
        .expect("Failed to start MySQL container")
}

#[tokio::test]
async fn test_sqlite_connection() {
    let config = DatabaseConfig::sqlite("test.db");

    let driver = Arc::new(SQLiteDriver);
    match SqlxQueryExecutor::new(driver, config.connection_url().unwrap().as_str()).await {
        Ok(executor) => {
            println!("✅ SQLite connection successful");

            let result = executor.query_raw("SELECT 1 as test", &[]).await;
            assert!(result.is_ok(), "SQLite query failed: {:?}", result.err());
            println!("✅ SQLite query successful");
        }
        Err(e) => {
            println!("❌ SQLite connection failed: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_postgres_connection() {
    let container = start_postgres_container().await;
    let port = container
        .get_host_port_ipv4(5432)
        .await
        .expect("Failed to get port");

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let config = DatabaseConfig::postgres("localhost", port, "my_postgres_db", "admin", "secret");

    let driver = Arc::new(PostgresDriver);
    let mut attempts = 0;
    const MAX_ATTEMPTS: u32 = 10;

    while attempts < MAX_ATTEMPTS {
        match SqlxQueryExecutor::new(
            Arc::clone(&driver),
            config.connection_url().unwrap().as_str(),
        )
        .await
        {
            Ok(executor) => {
                println!("✅ PostgreSQL connection successful");

                match executor.query_raw("SELECT 1 as test", &[]).await {
                    Ok(_) => {
                        println!("✅ PostgreSQL query successful");
                        return;
                    }
                    Err(e) => {
                        if attempts == MAX_ATTEMPTS - 1 {
                            panic!(
                                "PostgreSQL query failed after {} attempts: {:?}",
                                MAX_ATTEMPTS, e
                            );
                        }
                    }
                }
            }
            Err(e) => {
                if attempts == MAX_ATTEMPTS - 1 {
                    panic!(
                        "PostgreSQL connection failed after {} attempts: {:?}",
                        MAX_ATTEMPTS, e
                    );
                }
                println!(
                    "PostgreSQL connection attempt {} failed, retrying...",
                    attempts + 1
                );
            }
        }

        attempts += 1;
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }
}

#[tokio::test]
async fn test_mysql_connection() {
    let container = start_mysql_container().await;
    let port = container
        .get_host_port_ipv4(3306)
        .await
        .expect("Failed to get port");

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    let config = DatabaseConfig::mysql("localhost", port, "my_mysql_db", "admin", "secret");

    let driver = Arc::new(MySQLDriver);
    let mut attempts = 0;
    const MAX_ATTEMPTS: u32 = 15;

    while attempts < MAX_ATTEMPTS {
        match SqlxQueryExecutor::new(
            Arc::clone(&driver),
            config.connection_url().unwrap().as_str(),
        )
        .await
        {
            Ok(executor) => {
                println!("✅ MySQL connection successful");
                match executor.query_raw("SELECT 1 as test", &[]).await {
                    Ok(_) => {
                        println!("✅ MySQL query successful");
                        return;
                    }
                    Err(e) => {
                        if attempts == MAX_ATTEMPTS - 1 {
                            panic!(
                                "MySQL query failed after {} attempts: {:?}",
                                MAX_ATTEMPTS, e
                            );
                        }
                    }
                }
            }
            Err(e) => {
                if attempts == MAX_ATTEMPTS - 1 {
                    panic!(
                        "MySQL connection failed after {} attempts: {:?}",
                        MAX_ATTEMPTS, e
                    );
                }
                println!(
                    "MySQL connection attempt {} failed, retrying...",
                    attempts + 1
                );
            }
        }

        attempts += 1;
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }
}

#[tokio::test]
async fn test_database_config_validation() {
    let sqlite_config = DatabaseConfig::sqlite("test.db");
    assert!(sqlite_config.validate().is_ok());

    let postgres_config = DatabaseConfig::postgres("localhost", 5432, "test", "user", "pass");
    assert!(postgres_config.validate().is_ok());

    let mysql_config = DatabaseConfig::mysql("localhost", 3306, "test", "user", "pass");
    assert!(mysql_config.validate().is_ok());

    let invalid_sqlite = DatabaseConfig::sqlite("");
    assert!(invalid_sqlite.validate().is_err());

    let mut invalid_postgres = DatabaseConfig::postgres("", 5432, "test", "user", "pass");
    invalid_postgres.host = Some("".to_string());
    assert!(invalid_postgres.validate().is_err());

    let mut invalid_mysql = DatabaseConfig::mysql("localhost", 0, "test", "user", "pass");
    invalid_mysql.port = Some(0);
    assert!(invalid_mysql.validate().is_err());
}
