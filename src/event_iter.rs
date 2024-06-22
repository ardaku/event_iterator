use core::{
    ops::Deref,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{Map, Next};

/// An asynchronous lending iterator
///
/// Unlike iterators, the type must only be modified through interior mutability
/// during iteration.  This is to get around the limitation of not being able to
/// use [`Pin::as_mut()`] in some situations, due to the fact that events take
/// the lifetime of `Self`, resulting in insufficient lifetimes.
///
/// # Example
///
/// ```rust
#[doc = include_str!("../examples/stdin.rs")]
/// ```
pub trait EventIterator {
    /// The type of the events being iterated over
    type Event<'me>
    where
        Self: 'me;

    /// Attempt to pull out the next event of this event iterator, registering
    /// the current task for wakeup if the value is not yet available, and
    /// returning `None` if the event iterator is exhausted.
    ///
    /// # Return value
    ///
    /// There are several possible return values, each indicating a distinct
    /// event iterator state:
    ///
    /// - `Poll::Pending` means that this event iterator’s next value is not
    ///   ready yet.  Implementations will ensure that the current task will be
    ///   notified when the next value may be ready.
    /// - `Poll::Ready(Some(val))` means that the event iterator has
    ///   successfully produced a value, `val`, and may produce further values
    ///   on subsequent poll_next calls.
    /// - `Poll::Ready(None)` means that the event iterator has terminated, and
    ///   `poll_next()` should not be invoked again.
    ///
    /// # Panics
    ///
    /// Once an event iterator has finished (returned `Ready(None)` from
    /// `poll_next()`), calling its `poll_next()` method again may panic, block
    /// forever, or cause other kinds of problems; the `EventIterator` trait
    /// places no requirements on the effects of such a call. However, as the
    /// `poll_next()` method is not marked unsafe, Rust’s usual rules apply:
    /// calls must never cause undefined behavior (memory corruption, incorrect
    /// use of unsafe functions, or the like), regardless of the event
    /// iterator’s state.
    fn poll_next<'a>(
        self: Pin<&'a Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>>;

    /// Create a future that resolves to the next event in the event iterator.
    ///
    /// This is more flexible than [`next_unpinned()`](Self::next_unpinned), but
    /// often more verbose than needed.
    fn next<'a>(self: Pin<&'a Self>) -> Next<'a, Self>
    where
        Self: Sized,
    {
        Next::new(self)
    }

    /// Create a future that resolves to the next event in the event iterator.
    ///
    /// This is less flexible than [`next()`](Self::next), but avoids the need
    /// to handle pinning yourself.
    fn next_unpinned(&self) -> Next<'_, Self>
    where
        Self: Sized + Unpin,
    {
        Pin::new(self).next()
    }

    /// Return the bounds on the remaining length of the event iterator.
    ///
    /// Specifically, `size_hint()` returns a tuple where the first element is
    /// the lower bound, and the second element is the upper bound.
    ///
    /// The second half of the tuple that is returned is an
    /// <code>[Option]<[usize]></code>.  A `None` here means that either there
    /// is no known upper bound, or the upper bound is larger than [`usize`].
    ///
    /// # Implementation notes
    ///
    /// It is not enforced that an event iterator implementation yields the
    /// declared number of elements.  A buggy event iterator may yield less than
    /// the lower bound or more than the upper bound of elements.
    ///
    /// `size_hint()` is primarily intended to be used for optimizations such as
    /// reserving space for the events of the event iterator, but must not be
    /// trusted to e.g., omit bounds checks in unsafe code.  An incorrect
    /// implementation of `size_hint()` should not lead to memory safety
    /// violations.
    ///
    /// That said, the implementation should provide a correct estimation,
    /// because otherwise it would be a violation of the trait’s protocol.
    ///
    /// The default implementation returns `(0, None)` which is correct for any
    /// event iterator.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    /// Takes a closure and creates an event iterator which calls that closure
    /// on each event.
    ///
    /// `map()` transforms one event iterator into another, by means of its
    /// argument: something that implements [`FnMut`].  It produces a new event
    /// iterator which calls this closure on each event of the original event
    /// iterator.
    ///
    /// If you are good at thinking in types, you can think of `map()` like
    /// this: If you have an iterator that gives you elements of some type `A`,
    /// and you want an iterator of some other type `B`, you can use `map()`,
    /// passing a closure that takes an `A` and returns a `B`.
    ///
    /// `map()` is conceptually similar to an async for loop. However, as
    /// `map()` is lazy, it is best used when you’re already working with other
    /// event iterators.  If you’re doing some sort of looping for a side
    /// effect, it’s considered more idiomatic to use for than `map()`.
    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: for<'me> FnMut(Self::Event<'me>) -> B,
    {
        Map::new(self, f)
    }

    // TODO
    // filter
    // filter_map
    // enumerate
    // fuse
    // tear
    // inspect
    // take
    // take_while
}

impl<T> EventIterator for T
where
    T: Deref,
    T::Target: EventIterator + Unpin,
{
    type Event<'me> = <<T as Deref>::Target as EventIterator>::Event<'me>
        where Self: 'me;

    fn poll_next<'a>(
        self: Pin<&'a Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>> {
        Pin::new(&**self.get_ref()).poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (**self).size_hint()
    }
}
