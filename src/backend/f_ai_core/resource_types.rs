use tokio::sync::SemaphorePermit as TokioPermit;

pub struct SemaphorePermit<'a> {
    _permit: TokioPermit<'a>,
}

impl<'a> Drop for SemaphorePermit<'a> {
    fn drop(&mut self) {
        // Permit is automatically released when dropped
    }
} 