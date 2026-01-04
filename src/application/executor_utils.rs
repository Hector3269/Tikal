use crate::domain::repositories::executor::QueryExecutor;
use crate::domain::value_objects::Value;
use crate::domain::TikalResult;

pub async fn execute_batch<E: QueryExecutor>(
    executor: &E,
    queries: Vec<(String, Vec<Value>)>,
) -> TikalResult<Vec<u64>> {
    let mut results = Vec::new();

    for (sql, params) in queries {
        let rows_affected = executor.execute(&sql, params).await?;
        results.push(rows_affected);
    }

    Ok(results)
}

pub async fn execute_with_retry<E: QueryExecutor>(
    executor: &E,
    sql: &str,
    params: Vec<Value>,
    max_retries: usize,
) -> TikalResult<u64> {
    let mut last_error = None;

    for attempt in 0..=max_retries {
        match executor.execute(sql, params.clone()).await {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = Some(e);
                if attempt < max_retries {
                    tokio::time::sleep(std::time::Duration::from_millis(
                        100 * (attempt + 1) as u64,
                    ))
                    .await;
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| {
        crate::domain::error::TikalError::internal_error(
            "Unexpected error in retry logic",
            None,
        )
    }))
}