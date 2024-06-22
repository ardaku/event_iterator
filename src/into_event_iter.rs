use crate::EventIterator;

/// Trait for converting something into a `'static` event iterator
///
/// This is automatically implemented for all `'static` types that implement
/// [`EventIterator`].
pub trait IntoEventIterator {
    /// The type of the event yielded by the event iterator
    type Event<'me>
    where
        Self: 'me;
    /// The type of the resulting event iterator
    type IntoEventIter: for<'me> EventIterator<Event<'me> = Self::Event<'me>>
        + 'static;

    /// Convert `self` into an event iterator.
    fn into_event_iter(self) -> Self::IntoEventIter;
}

impl<I> IntoEventIterator for I
where
    I: EventIterator + 'static,
{
    type Event<'me> = I::Event<'me> where Self: 'me;
    type IntoEventIter = Self;

    fn into_event_iter(self) -> Self::IntoEventIter {
        self
    }
}
