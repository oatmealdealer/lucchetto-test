use lucchetto::{without_gvl, GvlSafe};
use std::future::Future;

pub struct GvlSafeFuture<'r, O, F>
where
    F: Future<Output = O>,
{
    runtime: &'r tokio::runtime::Runtime,
    future: F,
}

impl<'r, O, F> GvlSafe for GvlSafeFuture<'r, O, F> where F: Future<Output = O> {}

impl<'r, O, F> GvlSafeFuture<'r, O, F>
where
    F: Future<Output = O>,
{
    pub fn run(self) -> O {
        self.runtime.block_on(self.future)
    }
}

#[without_gvl]
fn run_future<'r, O, F>(future: GvlSafeFuture<'r, O, F>) -> O
where
    F: Future<Output = O>,
{
    future.run()
}
