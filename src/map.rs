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
    pub struct Map<I, F, E> {
        #[pin]
        ei: I,
        f: F,
        event: Option<E>,
    }
}

impl<I, F, E> Map<I, F, E> {
    pub(crate) fn new(ei: I, f: F) -> Self {
        let event = None;

        Self { ei, f, event }
    }
}

impl<I, F, E> fmt::Debug for Map<I, F, E>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Map")
            .field("ei", &self.ei)
            .finish_non_exhaustive()
    }
}

impl<E, I, F> EventIterator for Map<I, F, E>
where
    I: EventIterator,
    F: for<'me> FnMut(I::Event<'me>) -> E,
{
    type Event<'me>
        = &'me E
    where
        Self: 'me;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut this = self.project();
        let poll = this.ei.as_mut().poll(cx);

        if poll.is_ready() {
            (*this.event) = this.ei.event().map(this.f);
        }

        poll
    }

    fn event<'a>(self: Pin<&'a mut Self>) -> Option<Self::Event<'a>> {
        let this = self.project();

        this.event.as_ref()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.ei.size_hint()
    }
}
