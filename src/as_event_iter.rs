use core::{
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

/// Event iterator returned from [`AsEventIterator::as_event_iter()`]
///
/// See its documentation for more.
pub struct AsEventIter<'b, E>(&'b (dyn DynEventIter<'b, Event = E> + Unpin));

impl<'b, E> AsEventIter<'b, E> {
    /// Create a new `AsEventIter` from something implementing
    /// [`EventIterator`].
    pub fn new(ei: &'b (impl EventIterator<Event<'b> = E> + Unpin)) -> Self {
        Self(ei)
    }
}

impl<E> fmt::Debug for AsEventIter<'_, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AsEventIter")
            .field(&format_args!("_"))
            .finish()
    }
}

impl<E> EventIterator for AsEventIter<'_, E> {
    type Event<'me>
        = E
    where
        Self: 'me;

    fn poll_next(
        self: Pin<&Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'_>>> {
        Pin::new(&self.0).poll_next(cx)
    }
}

/// Trait for casting as an [`EventIterator`]
///
/// # Example
///
/// ```rust
#[doc = include_str!("../examples/as_event_iter.rs")]
/// ```
pub trait AsEventIterator<'b>: Unpin {
    /// The type of the events being iterated over
    type Event;

    /// Cast to an [`AsEventIter`].
    ///
    /// # Example
    ///
    /// ```rust
    #[doc = include_str!("../examples/tripple_buffer.rs")]
    /// ```
    fn as_event_iter(&'b self) -> AsEventIter<'b, Self::Event>;
}

impl<'b, T> AsEventIterator<'b> for T
where
    T: EventIterator + Unpin + 'b,
{
    type Event = <T as EventIterator>::Event<'b>;

    fn as_event_iter(&'b self) -> AsEventIter<'b, Self::Event> {
        AsEventIter::new(self)
    }
}

trait DynEventIter<'b> {
    type Event;

    fn poll_next(&'b self, cx: &mut Context<'_>) -> Poll<Option<Self::Event>>;
}

impl<'b, T> DynEventIter<'b> for T
where
    T: EventIterator + Unpin + 'b,
{
    type Event = <T as EventIterator>::Event<'b>;

    fn poll_next(&'b self, cx: &mut Context<'_>) -> Poll<Option<Self::Event>> {
        Pin::new(self).poll_next(cx)
    }
}
