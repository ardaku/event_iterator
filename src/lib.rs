//! Asynchronous lending iterator

#![no_std]
#![forbid(missing_docs, unsafe_code)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

use core::{
    future::Future,
    ops::{Deref, DerefMut},
    pin::Pin,
    task::{Context, Poll},
};

/// An asynchronous lending iterator
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
        self: Pin<&'a mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>>;

    /// Create a future that resolves to the next event in the event iterator.
    ///
    /// This is more flexible than [`next_unpinned()`](Self::next_unpinned), but
    /// often more verbose than needed.
    fn next<'a>(self: Pin<&'a mut Self>) -> Next<'a, Self> {
        Next(Some(self))
    }

    /// Create a future that resolves to the next event in the event iterator.
    ///
    /// This is less flexible than [`next()`](Self::next), but avoids the need
    /// to handle pinning yourself.
    fn next_unpinned(&mut self) -> Next<'_, Self>
    where
        Self: Unpin,
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
}

impl<T> EventIterator for T
where
    T: DerefMut + Unpin,
    T::Target: EventIterator + Unpin,
{
    type Event<'me> = <<T as Deref>::Target as EventIterator>::Event<'me> where Self: 'me;

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

/// Future for the [`next()`](EventIterator::next) and
/// [`next_unpinned()`](EventIterator::next_unpinned) methods
#[derive(Debug)]
pub struct Next<'a, Ei>(Option<Pin<&'a mut Ei>>)
where
    Ei: ?Sized;

impl<'a, Ei> Future for Next<'a, Ei>
where
    Ei: ?Sized + EventIterator,
{
    type Output = Option<Ei::Event<'a>>;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        let Some(ei) = self.0.take() else {
            return Poll::Ready(None);
        };

        ei.poll_next(cx)
    }
}

/// An event iterator that was created from iterator
///
/// This event iterator is created by the [`from_iter()`] function.  See it
/// documentation for more.
#[derive(Debug)]
pub struct FromIter<I>(I);

impl<I> Unpin for FromIter<I> {}

impl<I> EventIterator for FromIter<I>
where
    I: Iterator,
{
    type Event<'me> = <I as Iterator>::Item where I: 'me;

    fn poll_next<'a>(
        mut self: Pin<&'a mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>> {
        Poll::Ready(self.0.next())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

/// Convert an iterator into an async iterator.
pub fn from_iter<I>(iter: I) -> FromIter<<I as IntoIterator>::IntoIter>
where
    I: IntoIterator,
{
    FromIter(iter.into_iter())
}

/// Trait for converting something into an event iterator
pub trait IntoEventIterator {
    /// The type of the event yielded by the event iterator
    type Event<'me>
    where
        Self: 'me;
    /// The type of the resulting event iterator
    type IntoEventIter: for<'me> EventIterator<Event<'me> = Self::Event<'me>>
        + 'static;

    /// Convert `self` into an event iterator.
    fn into_event_iter(self) -> Self::IntoEventIter;
}

impl<I> IntoEventIterator for I
where
    I: EventIterator + 'static,
{
    type Event<'me> = I::Event<'me> where Self: 'me;
    type IntoEventIter = Self;

    fn into_event_iter(self) -> Self::IntoEventIter {
        self
    }
}
