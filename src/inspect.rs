use core::{
    cell::Cell,
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

/// Event iterator that calls a closure with a reference to each event
///
/// This `struct` is created by the [`EventIterator::map()`] method.  See its
/// documentation for more.
pub struct Inspect<I, F> {
    ei: I,
    f: Cell<Option<F>>,
}

impl<I, F> Inspect<I, F> {
    pub(crate) fn new(ei: I, f: F) -> Self {
        let f = Cell::new(Some(f));

        Self { ei, f }
    }
}

impl<I, F> fmt::Debug for Inspect<I, F>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Inspect")
            .field("ei", &self.ei)
            .finish_non_exhaustive()
    }
}

impl<I, F> EventIterator for Inspect<I, F>
where
    I: EventIterator + Unpin,
    F: for<'me> FnMut(&I::Event<'me>) + 'static + Unpin,
{
    type Event<'me> = I::Event<'me> where I: 'me;

    fn poll_next<'a>(
        self: Pin<&'a Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>> {
        let this = self.get_ref();
        let event = Pin::new(&this.ei).poll_next(cx);
        let Poll::Ready(Some(event)) = event else {
            return event;
        };

        if let Some(mut f) = this.f.take() {
            f(&event);
            this.f.set(Some(f));
        }

        Poll::Ready(Some(event))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.ei.size_hint()
    }
}
