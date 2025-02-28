use core::{
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

pin_project_lite::pin_project! {
    /// Event iterator that only yields elements while a predicate returns
    /// `true`
    ///
    /// This `struct` is created by the [`EventIterator::take_while()`] method.
    /// See its documentation for more.
    pub struct TakeWhile<I, P> {
        #[pin]
        ei: Option<I>,
        p: P,
    }
}

impl<I, P> TakeWhile<I, P> {
    pub(crate) fn new(ei: I, p: P) -> Self {
        let ei = Some(ei);

        Self { ei, p }
    }
}

impl<I, P> fmt::Debug for TakeWhile<I, P>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TakeWhile")
            .field("ei", &self.ei)
            .finish_non_exhaustive()
    }
}

impl<I, P> EventIterator for TakeWhile<I, P>
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
        let Some(mut ei) = this.ei.as_mut().as_pin_mut() else {
            return Poll::Ready(());
        };
        let Poll::Ready(()) = ei.as_mut().poll(cx) else {
            return Poll::Pending;
        };
        let Some(event) = ei.as_mut().event() else {
            return Poll::Ready(());
        };

        if !(this.p)(event) {
            this.ei.set(None);
        }

        Poll::Ready(())
    }

    fn event<'a>(self: Pin<&'a mut Self>) -> Option<Self::Event<'a>> {
        let this = self.project();

        this.ei.as_pin_mut()?.event()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let Some(ref ei) = self.ei else {
            return (0, None);
        };
        let (_, upper) = ei.size_hint();

        // Can't know a lower bound, due to the predicate
        (0, upper)
    }
}
