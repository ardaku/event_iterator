use core::{
    cell::Cell,
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

/// Event iterator that yields a single event by polling a future
///
/// This event iterator is created by the [`once()`] function.  See its
/// documentation for more.
pub struct Once<F>(Cell<Option<F>>);

impl<F> fmt::Debug for Once<F>
where
    F: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let future = self.0.take();
        let result = f.debug_tuple("Once").field(&future).finish();

        self.0.set(future);
        result
    }
}

impl<F> EventIterator for Once<F>
where
    F: Future + Unpin,
{
    type Event<'me> = F::Output where Self: 'me;

    fn poll_next<'a>(
        self: Pin<&'a Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>> {
        let Some(mut future) = self.0.take() else {
            return Poll::Ready(None);
        };

        if let Poll::Ready(output) = Pin::new(&mut future).poll(cx) {
            return Poll::Ready(Some(output));
        }

        self.0.set(Some(future));
        Poll::Pending
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let Some(future) = self.0.take() else {
            return (0, Some(0));
        };

        self.0.set(Some(future));
        (1, Some(1))
    }
}

/// Create an event iterator that yields a single event by polling a future.
///
/// This event iterator can be considered [fused](EventIterator::fuse).
///
/// # Example
///
/// ```rust
#[doc = include_str!("../examples/once.rs")]
/// ```
pub fn once<F>(f: F) -> Once<F>
where
    F: Future,
{
    Once(Cell::new(Some(f)))
}
