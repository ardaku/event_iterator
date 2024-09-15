use core::{
    cell::Cell,
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

/// Event iterator that only yields a specified number of events
///
/// This `struct` is created by the [`EventIterator::take()`] method.  See its
/// documentation for more.
pub struct Take<I> {
    ei: I,
    count: Cell<usize>,
}

impl<I> Take<I> {
    pub(crate) fn new(ei: I, count: usize) -> Self {
        let count = Cell::new(count);

        Self { ei, count }
    }
}

impl<I> fmt::Debug for Take<I>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Take")
            .field("ei", &self.ei)
            .field("count", &self.count)
            .finish()
    }
}

impl<I> EventIterator for Take<I>
where
    I: EventIterator + Unpin,
{
    type Event<'me> = I::Event<'me> where I: 'me;

    fn poll_next<'a>(
        self: Pin<&'a Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>> {
        let this = self.get_ref();
        let count = this.count.get();

        if count == 0 {
            return Poll::Ready(None);
        }

        let Poll::Ready(event) = Pin::new(&this.ei).poll_next(cx) else {
            return Poll::Pending;
        };

        this.count.set(count - 1);
        Poll::Ready(event)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = self.count.get();

        if count == 0 {
            return (0, Some(0));
        }

        let (lower, upper) = self.ei.size_hint();
        let lower = lower.min(count);
        let upper = match upper {
            Some(x) if x < count => Some(x),
            _ => Some(count),
        };

        (lower, upper)
    }
}
