use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

/// Future to get the next event in an [`EventIterator`]
///
/// This `struct` is created by the [`next()`](EventIterator::next) and
/// [`next_unpinned()`](EventIterator::next_unpinned) methods.  See their
/// documentation for more.
#[derive(Debug)]
pub struct Next<'a, Ei>(Option<Pin<&'a mut Ei>>);

impl<'a, Ei> Next<'a, Ei> {
    pub(crate) fn new(ei: Pin<&'a mut Ei>) -> Self {
        Self(Some(ei))
    }
}

impl<'a, Ei> Future for Next<'a, Ei>
where
    Ei: EventIterator,
{
    type Output = Option<Ei::Event<'a>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let Some(mut ei) = this.0.take() else {
            return Poll::Pending;
        };
        let Poll::Ready(()) = ei.as_mut().poll(cx) else {
            this.0 = Some(ei);
            return Poll::Pending;
        };

        Poll::Ready(ei.event())
    }
}
