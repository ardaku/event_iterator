use core::{
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

pin_project_lite::pin_project! {
    /// Event iterator that returns `Ready(None)` forever after it's finished
    ///
    /// An event iterator is finished after it first returns `Ready(None)`.
    ///
    /// This `struct` is created by the [`EventIterator::fuse()`] method.  See
    /// its documentation for more.
    #[repr(transparent)]
    pub struct Fuse<I> {
        #[pin]
        ei: Option<I>,
    }
}

impl<I> fmt::Debug for Fuse<I>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Fuse").field("ei", &self.ei).finish()
    }
}

impl<I> Fuse<I> {
    pub(crate) fn new(ei: I) -> Self {
        Self { ei: Some(ei) }
    }
}

impl<I> EventIterator for Fuse<I>
where
    I: EventIterator + Unpin,
{
    type Event<'me>
        = I::Event<'me>
    where
        I: 'me;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.project();
        let Some(ei) = this.ei.as_pin_mut() else {
            return Poll::Ready(());
        };

        ei.poll(cx)
    }

    fn event<'a>(self: Pin<&'a mut Self>) -> Option<Self::Event<'a>> {
        let mut this = self.project();
        let Some(ei) = this.ei.as_mut().as_pin_mut() else {
            return None;
        };

        if ei.event().is_none() {
            (*this.ei) = None;
            return None;
        }

        this.ei.as_pin_mut().and_then(|ei| ei.event())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.ei
            .as_ref()
            .map(|ei| ei.size_hint())
            .unwrap_or((0, Some(0)))
    }
}
