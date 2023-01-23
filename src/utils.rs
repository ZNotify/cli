use std::future::Future;
use std::io;
use clap_complete::{generate, Generator};
use async_trait::async_trait;
use clap::Command;

pub(crate) fn run_blocking<F: Future>(future: F) {
    tokio::runtime::Runtime::new().unwrap().block_on(future);
}

#[async_trait]
pub(crate) trait Action {
    async fn execute(&self);
}

pub(crate) fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
