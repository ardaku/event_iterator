use core::{
    fmt,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

/// Event iterator that yields nothing
///
/// This event iterator is created by the [`empty()`] function.  See its
/// documentation for more.
pub struct Empty<E>(PhantomData<E>);

impl<E> fmt::Debug for Empty<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Empty").field(&format_args!("_")).finish()
    }
}

impl<E> EventIterator for Empty<E> {
    type Event<'me>
        = E
    where
        Self: 'me;

    fn poll_next<'a>(
        self: Pin<&'a Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>> {
        Poll::Ready(None)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(0))
    }
}

/// Create an event iterator that yields nothing.
///
/// This event iterator can be considered [fused](EventIterator::fuse).
///
/// # Example
///
/// ```rust
#[doc = include_str!("../examples/empty.rs")]
/// ```
pub fn empty<E>() -> Empty<E> {
    Empty(PhantomData::<E>)
}
