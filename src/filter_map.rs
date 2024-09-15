use core::{
    cell::Cell,
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

/// Event iterator that uses a closure to both filter and map events
///
/// This `struct` is created by the [`EventIterator::filter_map()`] method.  See
/// its documentation for more.
pub struct FilterMap<I, F> {
    ei: I,
    f: Cell<Option<F>>,
}

impl<I, F> FilterMap<I, F> {
    pub(crate) fn new(ei: I, f: F) -> Self {
        let f = Cell::new(Some(f));

        Self { ei, f }
    }
}

impl<I, F> fmt::Debug for FilterMap<I, F>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FilterMap")
            .field("ei", &self.ei)
            .finish_non_exhaustive()
    }
}

impl<I, F, B> EventIterator for FilterMap<I, F>
where
    I: EventIterator + Unpin,
    F: for<'me> FnMut(I::Event<'me>) -> Option<B> + 'static + Unpin,
{
    type Event<'me> = B where I: 'me;

    fn poll_next<'a>(
        self: Pin<&'a Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>> {
        let this = self.get_ref();

        loop {
            let Poll::Ready(event) = Pin::new(&this.ei).poll_next(cx) else {
                break Poll::Pending;
            };
            let Some(event) = event else {
                break Poll::Ready(None);
            };
            let Some(mut f) = this.f.take() else {
                break Poll::Ready(None);
            };
            let event = f(event);

            this.f.set(Some(f));

            let Some(event) = event else { continue };

            break Poll::Ready(Some(event));
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.ei.size_hint();

        // Can't know a lower bound, due to the predicate
        (0, upper)
    }
}
