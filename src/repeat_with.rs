use core::{
    cell::Cell,
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::EventIterator;

/// Event iterator where each iteration repeats the provided closure
///
/// This event iterator is created by the [`repeat_with()`] function.  See its
/// documentation for more.
pub struct RepeatWith<F, G> {
    generator: Cell<Option<G>>,
    future: Cell<Option<F>>,
}

impl<F, G> fmt::Debug for RepeatWith<F, G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RepeatWith").finish_non_exhaustive()
    }
}

impl<E, F, G> EventIterator for RepeatWith<F, G>
where
    F: Future<Output = E> + Unpin,
    G: FnMut() -> F + Unpin,
{
    type Event<'me>
        = E
    where
        Self: 'me;

    fn poll_next<'a>(
        self: Pin<&'a Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>> {
        for _ in 0..2 {
            match self.future.take() {
                Some(mut future) => {
                    let Poll::Ready(output) = Pin::new(&mut future).poll(cx)
                    else {
                        self.future.set(Some(future));
                        return Poll::Pending;
                    };

                    return Poll::Ready(Some(output));
                }
                _ => {
                    self.generator.set(self.generator.take().map(
                        |mut r#gen| {
                            self.future.set(Some(r#gen()));
                            r#gen
                        },
                    ));
                }
            }
        }

        unreachable!()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }
}

/// Create an event iterator where each iteration repeats the provided closure.
///
/// # Example
///
/// ```rust
#[doc = include_str!("../examples/repeat_with.rs")]
/// ```
pub fn repeat_with<E, F, G>(r#gen: G) -> RepeatWith<F, G>
where
    F: Future<Output = E>,
    G: FnMut() -> F,
{
    RepeatWith {
        generator: Cell::new(Some(r#gen)),
        future: Cell::new(None),
    }
}
