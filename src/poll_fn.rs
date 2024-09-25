use core::{
    cell::Cell,
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

/// Event iterator that wraps a function returning [`Poll`]
///
/// This event iterator is created by the [`poll_fn()`] function.  See its
/// documentation for more.
pub struct PollFn<F>(Cell<Option<F>>);

impl<F> fmt::Debug for PollFn<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PollFn").field(&format_args!("_")).finish()
    }
}

impl<F, T> EventIterator for PollFn<F>
where
    F: FnMut(&mut Context<'_>) -> Poll<Option<T>> + Unpin,
{
    type Event<'me> = T where Self: 'me;

    fn poll_next<'a>(
        self: Pin<&'a Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>> {
        self.0
            .take()
            .map(|mut f| {
                let poll = f(cx);

                self.0.set(Some(f));
                poll
            })
            .unwrap()
    }
}

/// Create an event iterator that wraps a function returning [`Poll`].
///
/// Polling the event iterator delegates to the wrapped function.
///
/// # Example
///
/// ```rust
#[doc = include_str!("../examples/poll_fn.rs")]
/// ```
pub fn poll_fn<T, F>(f: F) -> PollFn<F>
where
    F: FnMut(&mut Context<'_>) -> Poll<Option<T>>,
{
    PollFn(Cell::new(Some(f)))
}
