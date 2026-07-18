use core::{
    fmt,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

/// Event iterator that never produces an event and never finishes
///
/// This event iterator is created by the [`pending()`] function.  See its
/// documentation for more.
pub struct Pending<E>(PhantomData<E>);

impl<E> fmt::Debug for Pending<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Pending").field(&format_args!("_")).finish()
    }
}

impl<E> EventIterator for Pending<E> {
    type Event<'me>
        = E
    where
        Self: 'me;

    fn poll_next<'a>(
        self: Pin<&'a Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>> {
        Poll::Pending
    }
}

/// Create an event iterator that never produces an event and never finishes.
///
/// This event iterator can be considered [torn](EventIterator::tear).
///
/// # Example
///
/// ```rust
#[doc = include_str!("../examples/pending.rs")]
/// ```
pub fn pending<E>() -> Pending<E> {
    Pending(PhantomData::<E>)
}
