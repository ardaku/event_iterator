extern crate alloc;

use alloc::boxed::Box;
use core::{
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

impl<S: ?Sized + EventIterator + Unpin> EventIterator for Box<S> {
    type Event<'me> = S::Event<'me> where S: 'me;

    fn poll_next<'a>(
        self: Pin<&'a mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>> {
        Pin::new(&mut **self.get_mut()).poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (**self).size_hint()
    }
}
