use core::{
    cell::Cell,
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

/// Event iterator that is always ready with an event
///
/// This event iterator is created by the [`ready()`] function.  See its
/// documentation for more.
pub struct Ready<F>(Cell<Option<F>>);

impl<F> fmt::Debug for Ready<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Ready").field(&format_args!("_")).finish()
    }
}

impl<F, E> EventIterator for Ready<F>
where
    F: FnMut() -> E + Unpin,
{
    type Event<'me>
        = E
    where
        Self: 'me;

    fn poll_next<'a>(
        self: Pin<&'a Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>> {
        self.0
            .take()
            .map(|mut f| {
                let poll = f();

                self.0.set(Some(f));
                Poll::Ready(Some(poll))
            })
            .unwrap()
    }
}

/// Create an event iterator that is always ready with an event.
///
/// Polling the event iterator delegates to the wrapped function.
///
/// # Example
///
/// ```rust
#[doc = include_str!("../examples/ready.rs")]
/// ```
pub fn ready<E, F>(f: F) -> Ready<F>
where
    F: FnMut() -> E,
{
    Ready(Cell::new(Some(f)))
}
