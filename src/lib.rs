pub struct BlockingFuture<F: FnOnce() -> T, T> {
    f: Option<F>,
}

impl<F: FnOnce() -> T, T> BlockingFuture<F, T> {
    pub fn new(f: F) -> Self {
        Self { f: Some(f) }
    }
}

impl<F: FnOnce() -> T, T> futures::Future for BlockingFuture<F, T> {
    type Item = T;
    type Error = tokio_threadpool::BlockingError;
    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        tokio_threadpool::blocking(|| self.f.take().expect("closure already used")())
    }
}
