use crate::EventIterator;

/// Trait for converting something into an event iterator
///
/// This is automatically implemented for all types that implement
/// [`EventIterator`].
pub trait IntoEventIterator<'me>: 'me {
    ///
    /// # Example
    ///
    /// ```rust
    #[doc = include_str!("../examples/into_ei.rs")]
    /// ```
    /// The type of the event yielded by the event iterator
    type Event<'a>
    where
        'me: 'a;
    /// The type of the resulting event iterator
    type IntoEventIter: EventIterator<Event<'me> = Self::Event<'me>>;

    /// Convert `self` into an event iterator.
    fn into_event_iter(self) -> Self::IntoEventIter;
}

impl<'me, I> IntoEventIterator<'me> for I
where
    I: EventIterator + 'me,
{
    type Event<'a> = I::Event<'a> where 'me: 'a;
    type IntoEventIter = I;

    fn into_event_iter(self) -> Self::IntoEventIter {
        self
    }
}
