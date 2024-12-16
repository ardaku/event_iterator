use core::{
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

pin_project_lite::pin_project! {
    /// Event iterator that only yields a specified number of events
    ///
    /// This `struct` is created by the [`EventIterator::take()`] method.  See
    /// its documentation for more.
    pub struct Take<I> {
        #[pin]
        ei: I,
        count: usize,
    }
}

impl<I> Take<I> {
    pub(crate) fn new(ei: I, count: usize) -> Self {
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
    type Event<'me>
        = I::Event<'me>
    where
        I: 'me;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.project();

        if *this.count == 0 {
            return Poll::Ready(());
        }

        let poll = this.ei.poll(cx);

        if poll.is_ready() {
            (*this.count) -= 1;
        }

        poll
    }

    fn event<'a>(self: Pin<&'a mut Self>) -> Option<Self::Event<'a>> {
        let this = self.project();

        if *this.count == 0 {
            return None;
        }

        this.ei.event()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = self.count;

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
