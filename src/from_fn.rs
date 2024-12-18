use core::{
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

pin_project_lite::pin_project! {
    /// Event iterator where each iteration calls the provided closure
    ///
    /// This event iterator is created by the [`from_fn()`] function.  See its
    /// documentation for more.
    pub struct FromFn<G, F>
    where
        F: Future,
        G: FnMut() -> F,
    {
        generator: G,
        #[pin]
        future: Option<F>,
        event: Option<F::Output>,
    }
}

impl<G, F> fmt::Debug for FromFn<G, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FromFn").finish_non_exhaustive()
    }
}

impl<G, F, E> EventIterator for FromFn<G, F>
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
pub fn from_fn<G, F, E>(generator: G) -> FromFn<G, F>
where
    F: Future<Output = Option<E>>,
    G: FnMut() -> F,
{
    FromFn {
        generator,
        future: None,
        event: None,
    }
}
