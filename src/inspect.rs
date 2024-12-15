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
        needs_inspection: bool,
    }
}

impl<I, F> Inspect<I, F> {
    pub(crate) fn new(ei: I, f: F) -> Self {
        let needs_inspection = true;

        Self {
            ei,
            f,
            needs_inspection,
        }
    }
}

impl<I, F> fmt::Debug for Inspect<I, F>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Inspect")
            .field("ei", &self.ei)
            .field("needs_inspection", &self.needs_inspection)
            .finish_non_exhaustive()
    }
}

impl<I, F> EventIterator for Inspect<I, F>
where
    I: EventIterator + Unpin,
    F: for<'me> FnMut(&I::Event<'me>) + 'static,
{
    type Event<'me>
        = I::Event<'me>
    where
        I: 'me;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        self.as_mut().event();

        let this = self.project();
        let poll = this.ei.poll(cx);

        if poll.is_ready() {
            (*this.needs_inspection) = true;
        }

        poll
    }

    fn event<'a>(self: Pin<&'a mut Self>) -> Option<Self::Event<'a>> {
        let this = self.project();
        let event = this.ei.event()?;

        if *this.needs_inspection {
            (*this.needs_inspection) = false;
            (*this.f)(&event);
        }

        Some(event)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.ei.size_hint()
    }
}
