use core::{
    cell::Cell,
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

/// Event iterator that filters the events of an event iterator with a predicate
///
/// This `struct` is created by the [`EventIterator::filter()`] method.  See its
/// documentation for more.
pub struct Filter<I, P> {
    ei: I,
    p: Cell<Option<P>>,
}

impl<I, P> Filter<I, P> {
    pub(crate) fn new(ei: I, p: P) -> Self {
        let p = Cell::new(Some(p));

        Self { ei, p }
    }
}

impl<I, P> fmt::Debug for Filter<I, P>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Filter")
            .field("ei", &self.ei)
            .finish_non_exhaustive()
    }
}

impl<I, P> EventIterator for Filter<I, P>
where
    I: EventIterator + Unpin,
    P: for<'me> FnMut(&I::Event<'me>) -> bool + 'static + Unpin,
{
    type Event<'me>
        = I::Event<'me>
    where
        I: 'me;

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
            let Some(mut predicate) = this.p.take() else {
                break Poll::Ready(None);
            };
            let should_yield = predicate(&event);

            this.p.set(Some(predicate));

            if should_yield {
                break Poll::Ready(Some(event));
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.ei.size_hint();

        // Can't know a lower bound, due to the predicate
        (0, upper)
    }
}
