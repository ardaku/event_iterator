use core::{
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

pin_project_lite::pin_project! {
    /// Event iterator that calls a closure with a reference to each event
    ///
    /// This `struct` is created by the [`EventIterator::inspect()`] method.
    /// See its documentation for more.
    pub struct Inspect<I, F> {
        #[pin]
        ei: I,
        f: F,
    }
}

impl<I, F> Inspect<I, F> {
    pub(crate) fn new(ei: I, f: F) -> Self {
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
    F: for<'me> FnMut(I::Event<'me>) + 'static,
{
    type Event<'me>
        = I::Event<'me>
    where
        I: 'me;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        self.as_mut().event();

        let mut this = self.project();
        let poll = this.ei.as_mut().poll(cx);

        if poll.is_ready() {
            if let Some(event) = this.ei.event() {
                (*this.f)(event);
            }
        }

        poll
    }

    fn event<'a>(self: Pin<&'a mut Self>) -> Option<Self::Event<'a>> {
        let this = self.project();

        this.ei.event()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.ei.size_hint()
    }
}
