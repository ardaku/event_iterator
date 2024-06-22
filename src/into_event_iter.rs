use crate::EventIterator;

/// Trait for converting something into an event iterator
///
/// This is automatically implemented for all types that implement
/// [`EventIterator`].
pub trait IntoEventIterator<'a, 'b>
where
    'b: 'a,
{
    /// The type of the event yielded by the event iterator
    type Event: 'a;
    /// The type of the resulting event iterator
    type IntoEventIter: EventIterator<Event<'a> = Self::Event> + 'b;

    /// Convert `self` into an event iterator.
    fn into_event_iter(self) -> Self::IntoEventIter;
}

impl<'a, 'b, I> IntoEventIterator<'a, 'b> for I
where
    I: EventIterator + 'b,
    'b: 'a,
{
    type Event = I::Event<'a> where <I as EventIterator>::Event<'a>: 'a;
    type IntoEventIter = Self;

    fn into_event_iter(self) -> Self::IntoEventIter {
        self
    }
}
