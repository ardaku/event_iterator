use core::{
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{EventIterator, LendAs};

pin_project_lite::pin_project! {
    /// Event iterator that maps the events with a closure
    ///
    /// This `struct` is created by the [`EventIterator::map()`] method.
    /// See its documentation for more.
    pub struct Map<I, L> {
        #[pin]
        ei: I,
        lend_as : L,
    }
}

impl<I, L> Map<I, L> {
    pub(crate) fn new(ei: I, lend_as: L) -> Self {
        Self { ei, lend_as }
    }
}

impl<I, L> fmt::Debug for Map<I, L>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Map")
            .field("ei", &self.ei)
            .finish_non_exhaustive()
    }
}

impl<I, L> EventIterator for Map<I, L>
where
    I: EventIterator,
    L: for<'me> LendAs<From<'me> = I::Event<'me>> + Copy,
    for<'a> L::Into<'a>: Copy,
{
    type Event<'me>
        = L::Into<'me>
    where
        Self: 'me;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut this = self.project();

        this.ei.as_mut().poll(cx)
    }

    fn event<'a>(self: Pin<&'a mut Self>) -> Option<Self::Event<'a>> {
        let this = self.project();

        Some(this.lend_as.lend_as(this.ei.event()?))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.ei.size_hint()
    }
}
