use core::{
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

/// Event iterator that endlessly repeats a single event
///
/// This event iterator is created by the [`repeat()`] function.  See its
/// documentation for more.
pub struct Repeat<E>(E);

impl<E> fmt::Debug for Repeat<E>
where
    E: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Repeat").field(&self.0).finish()
    }
}

impl<E> EventIterator for Repeat<E>
where
    E: Clone,
{
    type Event<'me>
        = E
    where
        Self: 'me;

    fn poll_next<'a>(
        self: Pin<&'a Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>> {
        Poll::Ready(Some(self.0.clone()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }
}

/// Create an event iterator that endlessly repeats a single event.
///
/// # Example
///
/// ```rust
#[doc = include_str!("../examples/repeat.rs")]
/// ```
pub fn repeat<E>(event: E) -> Repeat<E>
where
    E: Clone,
{
    Repeat(event)
}
