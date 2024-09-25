use core::{
    cell::Cell,
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

/// Event iterator that yields a single event
///
/// This event iterator is created by the [`once()`] function.  See its
/// documentation for more.
pub struct Once<E>(Cell<Option<E>>);

impl<E> fmt::Debug for Once<E>
where
    E: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let event = self.0.take();
        let result = f.debug_tuple("Once").field(&event).finish();

        self.0.set(event);
        result
    }
}

impl<E> EventIterator for Once<E> {
    type Event<'me> = E where Self: 'me;

    fn poll_next<'a>(
        self: Pin<&'a Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>> {
        Poll::Ready(self.0.take())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let Some(future) = self.0.take() else {
            return (0, Some(0));
        };

        self.0.set(Some(future));
        (1, Some(1))
    }
}

/// Create an event iterator that yields a single event.
///
/// This event iterator can be considered [fused](EventIterator::fuse).
///
/// # Example
///
/// ```rust
#[doc = include_str!("../examples/once.rs")]
/// ```
pub fn once<E>(event: E) -> Once<E> {
    Once(Cell::new(Some(event)))
}
