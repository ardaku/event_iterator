use core::{
    fmt,
    iter::Peekable,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{consts::EVENT_BEFORE_POLL, EventIterator};

/// Event iterator of references that was created from an iterator
///
/// This event iterator is created by the [`from_iter_ref()`] function.  See its
/// documentation for more.
pub struct FromIterRef<I>
where
    I: Iterator,
{
    iter: Peekable<I>,
    started: bool,
}

impl<I> fmt::Debug for FromIterRef<I>
where
    I: Iterator + fmt::Debug,
    <I as Iterator>::Item: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("FromIterRef").field(&self.iter).finish()
    }
}

impl<I> EventIterator for FromIterRef<I>
where
    I: Iterator + Unpin,
    <I as Iterator>::Item: Unpin,
{
    type Event<'me>
        = &'me <I as Iterator>::Item
    where
        I: 'me;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        let this = self.get_mut();

        if this.started {
            _ = this.iter.next();
        } else {
            this.started = true;
        }

        Poll::Ready(())
    }

    fn event<'a>(self: Pin<&'a mut Self>) -> Option<Self::Event<'a>> {
        let this = self.get_mut();

        if !this.started {
            panic!("{EVENT_BEFORE_POLL}");
        }

        this.iter.peek()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();

        if self.started {
            (lower.saturating_sub(1), upper.map(|n| n.saturating_sub(1)))
        } else {
            (lower, upper)
        }
    }
}

/// Convert an iterator into an event iterator of references.
///
/// # Example
///
/// ```rust
#[doc = include_str!("../examples/from_iter_ref.rs")]
/// ```
/// 
/// Output:
/// ```console
/// 1
/// 2
/// 3
/// 4
/// 5
/// ```
/// 
/// # Panics
///
/// The returned event iterator might panic if [`EventIterator::event()`] is
/// called before [`EventIterator::poll()`].
pub fn from_iter_ref<I>(iter: I) -> FromIterRef<<I as IntoIterator>::IntoIter>
where
    I: IntoIterator,
{
    FromIterRef {
        iter: iter.into_iter().peekable(),
        started: false,
    }
}
