use tokio::sync::SemaphorePermit as TokioPermit;

pub struct SemaphorePermit<'a> {
    _permit: TokioPermit<'a>,
}

impl<'a> SemaphorePermit<'a> {
    pub fn new(permit: TokioPermit<'a>) -> Self {
        Self { _permit: permit }
    }
}

impl<'a> Drop for SemaphorePermit<'a> {
    fn drop(&mut self) {
        // Permit is automatically released when dropped
    }
} 