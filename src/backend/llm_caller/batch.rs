use tokio::sync::Semaphore;
use futures::future;

struct BatchProcessor {
    concurrency_limit: usize,
    semaphore: Arc<Semaphore>,
}

impl BatchProcessor {
    async fn process_batch<T, F, Fut>(&self, items: Vec<T>, operation: F) -> Result<Vec<Result<()>>> 
    where
        F: Fn(T) -> Fut + Clone + Send + 'static,
        Fut: Future<Output = Result<()>> + Send,
        T: Send + 'static,
    {
        let futures = items.into_iter().map(|item| {
            let permit = self.semaphore.clone().acquire_owned();
            let op = operation.clone();
            
            async move {
                let _permit = permit.await.map_err(|e| 
                    DatabaseError::Concurrency(e.to_string()))?;
                op(item).await
            }
        });

        future::join_all(futures).await
    }
}
