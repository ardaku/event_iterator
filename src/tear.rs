use core::{
    cell::Cell,
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

/// Event iterator that yields `Pending` forever after yielding `Ready(None)`
/// once
///
/// This `struct` is created by the [`EventIterator::tear()`] method.  See
/// its documentation for more.
pub struct Tear<I> {
    ei: I,
    ended: Cell<bool>,
}

impl<I> fmt::Debug for Tear<I>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tear")
            .field("ei", &self.ei)
            .field("ended", &self.ended)
            .finish()
    }
}

impl<I> Tear<I> {
    pub(crate) fn new(ei: I) -> Self {
        let ended = Cell::new(false);

        Self { ei, ended }
    }
}

impl<I> EventIterator for Tear<I>
where
    I: EventIterator + Unpin,
{
    type Event<'me> = I::Event<'me> where I: 'me;

    fn poll_next<'a>(
        self: Pin<&'a Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>> {
        let this = self.get_ref();

        if this.ended.get() {
            return Poll::Pending;
        }

        let poll = Pin::new(&this.ei).poll_next(cx);

        if let Poll::Ready(None) = poll {
            this.ended.set(true);
        }

        poll
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.ei.size_hint()
    }
}
