use core::{
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

pin_project_lite::pin_project! {
    /// Event iterator that maps the events with a closure
    ///
    /// This `struct` is created by the [`EventIterator::map()`] method.  See
    /// its documentation for more.
    pub struct Map<I, F> {
        #[pin]
        ei: I,
        f: F,
    }
}

impl<I, F> Map<I, F> {
    pub(crate) fn new(ei: I, f: F) -> Self {
        Self { ei, f }
    }
}

impl<I, F> fmt::Debug for Map<I, F>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Map")
            .field("ei", &self.ei)
            .finish_non_exhaustive()
    }
}

impl<B, I, F> EventIterator for Map<I, F>
where
    I: EventIterator + Unpin,
    F: for<'me> Fn(I::Event<'me>) -> B + 'static,
{
    type Event<'me>
        = B
    where
        I: 'me;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.project();

        this.ei.poll(cx)
    }

    fn event<'a>(self: Pin<&'a mut Self>) -> Option<Self::Event<'a>> {
        let this = self.project();

        this.ei.event().map(this.f)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.ei.size_hint()
    }
}
