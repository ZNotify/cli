use std::future::Future;

pub(crate) fn run_blocking<F: Future>(future: F) {
    tokio::runtime::Runtime::new().unwrap().block_on(future);
}