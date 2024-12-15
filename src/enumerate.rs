use core::{
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

pin_project_lite::pin_project! {
    /// Event iterator that yields the current count and event during iteration
    ///
    /// This `struct` is created by the [`EventIterator::enumerate()`] method.
    /// See its documentation for more.
    pub struct Enumerate<I> {
        #[pin]
        ei: I,
        count: usize,
    }
}

impl<I> Enumerate<I> {
    pub(crate) fn new(ei: I) -> Self {
        let count = 0;

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

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.project();
        let poll = this.ei.poll(cx);

        (*this.count) += 1;
        poll
    }

    fn event<'a>(self: Pin<&'a mut Self>) -> Option<Self::Event<'a>> {
        let this = self.project();
        let count = *this.count;

        this.ei.event().map(|e| (count, e))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.ei.size_hint()
    }
}
