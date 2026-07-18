use core::{
    cell::Cell,
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

/// Event iterator that yields the current count and event during iteration
///
/// This `struct` is created by the [`EventIterator::enumerate()`] method.  See
/// its documentation for more.
pub struct Enumerate<I> {
    ei: I,
    count: Cell<usize>,
}

impl<I> Enumerate<I> {
    pub(crate) fn new(ei: I) -> Self {
        let count = Cell::new(0);

        Self { ei, count }
    }
}

impl<I> fmt::Debug for Enumerate<I>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Map")
            .field("ei", &self.ei)
            .field("count", &self.count)
            .finish()
    }
}

impl<I> EventIterator for Enumerate<I>
where
    I: EventIterator + Unpin,
{
    type Event<'me>
        = (usize, I::Event<'me>)
    where
        I: 'me;

    fn poll_next<'a>(
        self: Pin<&'a Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>> {
        let this = self.get_ref();
        let Poll::Ready(event) = Pin::new(&this.ei).poll_next(cx) else {
            return Poll::Pending;
        };
        let count = this.count.get();

        this.count.set(count + 1);
        Poll::Ready(event.map(|e| (count, e)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.ei.size_hint()
    }
}
