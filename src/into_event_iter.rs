use crate::EventIterator;

/// Trait for converting something into an event iterator
///
/// This is automatically implemented for all types that implement
/// [`EventIterator`].
pub trait IntoEventIterator<'a> where Self: 'a {
    /// The type of the event yielded by the event iterator
    type Event<'me>
    where
        Self: 'me + 'a;
    /// The type of the resulting event iterator
    type IntoEventIter: EventIterator<Event<'a> = Self::Event<'a>>;

    /// Convert `self` into an event iterator.
    fn into_event_iter(self) -> Self::IntoEventIter;
}

impl<'a, I> IntoEventIterator<'a> for I
where
    I: EventIterator + 'a,
{
    type Event<'me> = I::Event<'me> where Self: 'me;
    type IntoEventIter = Self;

    fn into_event_iter(self) -> Self::IntoEventIter {
        self
    }
}
