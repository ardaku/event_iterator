use core::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

pub struct AsEventIter<'a, 'b, E>(&'a dyn AsEventIterator<'b, Event = E>);

impl<'a: 'b, 'b, E> EventIterator for AsEventIter<'a, 'b, E> {
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
    fn poll_next(
        self: &'b Self,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event>>;

    /// Cast to an `AsEventIterator` trait object
    fn as_event_iter(&self) -> &dyn AsEventIterator<'b, Event = Self::Event>;

    /// Cast to an `AsEventIterator` trait object
    fn as_event_iter2(&self) -> AsEventIter<'_, 'b, Self::Event>;
}

impl<'b, T> AsEventIterator<'b> for T
where
    T: EventIterator + Unpin + 'b,
{
    type Event = <T as EventIterator>::Event<'b>;

    fn poll_next(
        self: &'b Self,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event>> {
        Pin::new(self).poll_next(cx)
    }

    fn as_event_iter(&self) -> &dyn AsEventIterator<'b, Event = Self::Event> {
        self
    }

    fn as_event_iter2(&self) -> AsEventIter<'_, 'b, Self::Event> {
        AsEventIter(self)
    }
}

impl<E> EventIterator for dyn AsEventIterator<'_, Event = E> {
    type Event<'me> = E where Self: 'me;

    fn poll_next(
        self: Pin<&Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'_>>> {
        self.as_event_iter().poll_next(cx)
    }
}
