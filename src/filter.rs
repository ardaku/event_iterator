use core::{
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

pin_project_lite::pin_project! {
    /// Event iterator that filters the events of an event iterator with a
    /// predicate
    ///
    /// This `struct` is created by the [`EventIterator::filter()`] method.  See
    /// its documentation for more.
    pub struct Filter<I, P> {
        #[pin]
        ei: I,
        p: P,
    }
}

impl<I, P> Filter<I, P> {
    pub(crate) fn new(ei: I, p: P) -> Self {
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
    I: EventIterator,
    P: for<'me> FnMut(I::Event<'me>) -> bool,
{
    type Event<'me>
        = I::Event<'me>
    where
        I: 'me,
        P: 'me;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut this = self.project();

        loop {
            let Poll::Ready(()) = this.ei.as_mut().poll(cx) else {
                break Poll::Pending;
            };
            let Some(event) = this.ei.as_mut().event() else {
                break Poll::Ready(());
            };

            if (this.p)(event) {
                break Poll::Ready(());
            }
        }
    }

    fn event<'a>(self: Pin<&'a mut Self>) -> Option<Self::Event<'a>> {
        let this = self.project();

        this.ei.event()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.ei.size_hint();

        // Can't know a lower bound, due to the predicate
        (0, upper)
    }
}
