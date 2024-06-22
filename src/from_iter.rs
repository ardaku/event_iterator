use core::{
    cell::Cell,
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

/// An event iterator that was created from an iterator
///
/// This event iterator is created by the [`from_iter()`] function.  See it
/// documentation for more.
pub struct FromIter<I>(Cell<Option<I>>);

impl<I> fmt::Debug for FromIter<I>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.0.take();
        let result = f.debug_tuple("FromIter").field(&iter).finish();

        self.0.set(iter);
        result
    }
}

impl<I> Unpin for FromIter<I> {}

impl<I> EventIterator for FromIter<I>
where
    I: Iterator,
{
    type Event<'me> = <I as Iterator>::Item where I: 'me;

    fn poll_next<'a>(
        self: Pin<&'a Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>> {
        Poll::Ready(self.0.take().and_then(|mut iter| {
            let item = iter.next()?;

            self.0.set(Some(iter));
            Some(item)
        }))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0
            .take()
            .and_then(|iter| {
                let size = iter.size_hint();

                self.0.set(Some(iter));
                Some(size)
            })
            .unwrap_or((0, Some(0)))
    }
}

/// Convert an iterator into an event iterator.
pub fn from_iter<I>(iter: I) -> FromIter<<I as IntoIterator>::IntoIter>
where
    I: IntoIterator,
{
    FromIter(Cell::new(iter.into_iter().into()))
}
