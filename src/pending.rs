use core::{
    fmt,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{consts::EVENT_BEFORE_POLL, EventIterator};

/// [Torn](crate::Tear) event iterator that never produces an event and never
/// finishes
///
/// This event iterator is created by the [`pending()`] function.  See its
/// documentation for more.
pub struct Pending<E>(PhantomData<E>)
where
    E: Copy;

impl<E> fmt::Debug for Pending<E>
where
    E: Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Pending").field(&format_args!("_")).finish()
    }
}

impl<E> EventIterator for Pending<E>
where
    E: Copy,
{
    type Event<'me>
        = E
    where
        Self: 'me;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        Poll::Pending
    }

    fn event<'a>(self: Pin<&'a mut Self>) -> Option<Self::Event<'a>> {
        panic!("{EVENT_BEFORE_POLL}");
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
pub fn pending<E>() -> Pending<E>
where
    E: Copy,
{
    Pending(PhantomData::<E>)
}
