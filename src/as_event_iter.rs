use core::{
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

/// Event iterator returned from [`AsEventIterator::as_event_iter()`]
///
/// See its documentation for more.
pub struct AsEventIter<'a, 'b, E>(&'a dyn AsEventIterator<'b, Event = E>);

impl<'a, 'b, E> AsEventIter<'a, 'b, E> {
    /// Create a new `AsEventIter` from something implementing
    /// [`EventIterator`].
    pub fn new(ei: &'a (impl EventIterator<Event<'b> = E> + Unpin + 'b)) -> Self {
        Self(ei)
    }
}

impl<E> fmt::Debug for AsEventIter<'_, '_, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AsEventIter")
            .field(&format_args!("_"))
            .finish()
    }
}

impl<'a, 'b, E> EventIterator for AsEventIter<'a, 'b, E>
where
    'a: 'b,
{
    type Event<'me> = E where Self: 'me;

    fn poll_next(
        self: Pin<&Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'_>>> {
        self.0.poll_next(cx)
    }
}

/// Trait for casting as an [`EventIterator`]
pub trait AsEventIterator<'b>: Unpin {
    /// The type of the events being iterated over
    type Event;

    /// Attempt to pull out the next event of this event iterator, registering
    /// the current task for wakeup if the value is not yet available, and
    /// returning `None` if the event iterator is exhausted.
    ///
    /// See [`EventIterator::poll_next()`]'s documentation for more.
    fn poll_next(&'b self, cx: &mut Context<'_>) -> Poll<Option<Self::Event>>;

    /// Cast to an `AsEventIterator` trait object
    fn as_event_iter(&self) -> AsEventIter<'_, 'b, Self::Event>;
}

impl<'b, T> AsEventIterator<'b> for T
where
    T: EventIterator + Unpin + 'b,
{
    type Event = <T as EventIterator>::Event<'b>;

    fn poll_next(&'b self, cx: &mut Context<'_>) -> Poll<Option<Self::Event>> {
        Pin::new(self).poll_next(cx)
    }

    fn as_event_iter(&self) -> AsEventIter<'_, 'b, Self::Event> {
        AsEventIter(self)
    }
}
