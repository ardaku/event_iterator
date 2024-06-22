use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

/// Future for the [`next()`](EventIterator::next) and
/// [`next_unpinned()`](EventIterator::next_unpinned) methods
#[derive(Debug)]
pub struct Next<'a, Ei>(Option<Pin<&'a Ei>>);

impl<'a, Ei> Next<'a, Ei> {
    pub(crate) fn new(ei: Pin<&'a Ei>) -> Self {
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
        let Some(ei) = this.0.as_ref() else {
            return Poll::Ready(None);
        };
        let output = ei.poll_next(cx);

        if output.is_ready() {
            this.0 = None;
        }

        output
    }
}
