use core::{
    cell::Cell,
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

/// An event iterator that maps the events with f.
///
/// This `struct` is created by the [`EventIterator::map()`] method.  See its
/// documentation for more.
pub struct Map<I, F> {
    ei: I,
    f: Cell<Option<F>>,
}

impl<I, F> Map<I, F> {
    pub(crate) fn new(ei: I, f: F) -> Self {
        let f = Cell::new(Some(f));

        Self { ei, f }
    }
}

impl<I, F> fmt::Debug for Map<I, F>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Map")
            .field("ei", &self.ei)
            .finish_non_exhaustive()
    }
}

impl<B, I, F> EventIterator for Map<I, F>
where
    I: EventIterator + Unpin,
    F: for<'me> FnMut(I::Event<'me>) -> B + 'static + Unpin,
{
    type Event<'me> = B where I: 'me;

    fn poll_next<'a>(
        self: Pin<&'a Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>> {
        let this = self.get_ref();
        let Poll::Ready(item) = Pin::new(&this.ei).poll_next(cx) else {
            return Poll::Pending;
        };

        Poll::Ready(this.f.take().and_then(|mut f| {
            let event = item.map(&mut f);

            this.f.set(Some(f));
            event
        }))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.ei.size_hint()
    }
}
