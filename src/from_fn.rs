use core::{
    cell::Cell,
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

/// Event iterator where each iteration calls the provided closure
///
/// This event iterator is created by the [`from_fn()`] function.  See its
/// documentation for more.
pub struct FromFn<F, G> {
    generator: Cell<Option<G>>,
    future: Cell<Option<F>>,
}

impl<F, G> fmt::Debug for FromFn<F, G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FromFn").finish_non_exhaustive()
    }
}

impl<E, F, G> EventIterator for FromFn<F, G>
where
    F: Future<Output = Option<E>> + Unpin,
    G: FnMut() -> F + Unpin,
{
    type Event<'me> = E where Self: 'me;

    fn poll_next<'a>(
        self: Pin<&'a Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>> {
        for _ in 0..2 {
            if let Some(mut future) = self.future.take() {
                let Poll::Ready(output) = Pin::new(&mut future).poll(cx) else {
                    self.future.set(Some(future));
                    return Poll::Pending;
                };

                return Poll::Ready(output);
            } else {
                self.generator.set(self.generator.take().map(|mut gen| {
                    self.future.set(Some(gen()));
                    gen
                }));
            }
        }

        unreachable!()
    }
}

/// Create an event iterator where each iteration calls the provided closure.
///
/// # Example
///
/// ```rust
#[doc = include_str!("../examples/from_fn.rs")]
/// ```
pub fn from_fn<E, F, G>(gen: G) -> FromFn<F, G>
where
    F: Future<Output = Option<E>>,
    G: FnMut() -> F,
{
    FromFn {
        generator: Cell::new(Some(gen)),
        future: Cell::new(None),
    }
}
