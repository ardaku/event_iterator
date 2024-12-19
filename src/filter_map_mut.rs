use core::{
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

pin_project_lite::pin_project! {
    /// Event iterator that uses a closure to both filter and map a reference to
    /// events
    ///
    /// This `struct` is created by the [`EventIterator::filter_map_mut()`]
    /// method.  See its documentation for more.
    pub struct FilterMapMut<I, F, E> {
        #[pin]
        ei: I,
        f: F,
        event: Option<E>,
    }
}

impl<I, F, E> FilterMapMut<I, F, E> {
    pub(crate) fn new(ei: I, f: F) -> Self {
        let event = None;

        Self { ei, f, event }
    }
}

impl<I, F, E> fmt::Debug for FilterMapMut<I, F, E>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FilterMapMut")
            .field("ei", &self.ei)
            .finish_non_exhaustive()
    }
}

impl<I, F, E> EventIterator for FilterMapMut<I, F, E>
where
    I: EventIterator,
    F: for<'me> FnMut(I::Event<'me>) -> Option<E>,
{
    type Event<'me>
        = &'me mut E
    where
        I: 'me,
        E: 'me,
        F: 'me;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut this = self.project();

        loop {
            let Poll::Ready(()) = this.ei.as_mut().poll(cx) else {
                break Poll::Pending;
            };
            let Some(event) = this.ei.as_mut().event() else {
                (*this.event) = None;
                break Poll::Ready(());
            };

            (*this.event) = (this.f)(event);

            if this.event.is_some() {
                break Poll::Ready(());
            }
        }
    }

    fn event<'a>(self: Pin<&'a mut Self>) -> Option<Self::Event<'a>> {
        let this = self.project();

        this.event.as_mut()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.ei.size_hint();

        // Can't know a lower bound, due to the predicate
        (0, upper)
    }
}
