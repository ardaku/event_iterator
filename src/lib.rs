//! Asynchronous lending iterator
//!
//! # Getting Started
//!
//! The following exaple shows how to implement and use an event iterator to
//! print to stdout:
//!
//! ```console
//! Hello, world!
//! Hello, again!
//! ```
//!
//! ## Code
//!
//! ```rust
#![doc = include_str!("../examples/stdout.rs")]
//! ```

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
    cell::Cell,
    fmt,
    future::Future,
    ops::Deref,
    pin::Pin,
    task::{Context, Poll},
};

/// An asynchronous lending iterator
///
/// Unlike iterators, the type must only be modified through interior mutability
/// during iteration.  This is to get around the limitation of not being able to
/// use [`Pin::as_mut()`] in some situations, due to the fact that events take
/// the lifetime of `Self`, resulting in insufficient lifetimes.
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
    fn next<'a>(self: Pin<&'a Self>) -> Next<'a, Self> {
        Next(Some(self))
    }

    /// Create a future that resolves to the next event in the event iterator.
    ///
    /// This is less flexible than [`next()`](Self::next), but avoids the need
    /// to handle pinning yourself.
    fn next_unpinned(&self) -> Next<'_, Self>
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
        Self: Sized + EventIterator,
        F: for<'me> FnMut(Self::Event<'me>) -> B,
    {
        Map {
            ei: self,
            f: Cell::new(f.into()),
        }
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

/// Future for the [`next()`](EventIterator::next) and
/// [`next_unpinned()`](EventIterator::next_unpinned) methods
#[derive(Debug)]
pub struct Next<'a, Ei>(Option<Pin<&'a Ei>>)
where
    Ei: ?Sized;

impl<'a, Ei> Future for Next<'a, Ei>
where
    Ei: EventIterator + ?Sized,
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

/// An event iterator that maps the events with f.
///
/// This `struct` is created by the [`EventIterator::map()`] method.  See its
/// documentation for more.
pub struct Map<I, F> {
    ei: I,
    f: Cell<Option<F>>,
}

impl<I, F> fmt::Debug for Map<I, F>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Map")
            .field("ei", &self.ei)
            .finish_non_exhaustive()
    }
}

impl<B, I, F> EventIterator for Map<I, F>
where
    I: EventIterator + Unpin,
    F: for<'me> FnMut(I::Event<'me>) -> B + 'static + Unpin,
{
    type Event<'me> = B where I: 'me;

    fn poll_next<'a>(
        self: Pin<&'a Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>> {
        let this = self.get_ref();
        let Poll::Ready(item) = Pin::new(&this.ei).poll_next(cx) else {
            return Poll::Pending;
        };

        Poll::Ready(this.f.take().and_then(|mut f| {
            let event = item.map(&mut f);

            this.f.set(Some(f));
            event
        }))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.ei.size_hint()
    }
}

/// An event iterator that was created from an iterator
///
/// This event iterator is created by the [`from_iter()`] function.  See it
/// documentation for more.
pub struct FromIter<I>(Cell<Option<I>>);

impl<I> fmt::Debug for FromIter<I>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.0.take();
        let result = f.debug_tuple("FromIter").field(&iter).finish();

        self.0.set(iter);
        result
    }
}

impl<I> Unpin for FromIter<I> {}

impl<I> EventIterator for FromIter<I>
where
    I: Iterator,
{
    type Event<'me> = <I as Iterator>::Item where I: 'me;

    fn poll_next<'a>(
        self: Pin<&'a Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>> {
        Poll::Ready(self.0.take().and_then(|mut iter| {
            let item = iter.next()?;

            self.0.set(Some(iter));
            Some(item)
        }))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0
            .take()
            .and_then(|iter| {
                let size = iter.size_hint();

                self.0.set(Some(iter));
                Some(size)
            })
            .unwrap_or((0, Some(0)))
    }
}

/// Convert an iterator into an event iterator.
pub fn from_iter<I>(iter: I) -> FromIter<<I as IntoIterator>::IntoIter>
where
    I: IntoIterator,
{
    FromIter(Cell::new(iter.into_iter().into()))
}

/// Trait for converting something into a `'static` event iterator
///
/// This is automatically implemented for all `'static` types that implement
/// [`EventIterator`].
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

// TODO
//
//  /// Create an event iterator which never produces events.
//  pub fn pending<E>() -> Pending<E>;
//  /// Event iterator which is empty (always returns `Ready(None)`).
//  pub fn empty<E>() -> Empty<E>;
//  /// Create an event iterator that wraps a function returning [`Poll`].
//  pub fn poll_fn<T, F>(f: F) -> PollFn<F>
//  where
//      F: FnMut(&mut Context<'_>) -> Poll<Option<T>>;
//  /// Create an event iterator where each iteration calls the provided closure
//  pub fn from_fn<E, F: Future<Output = Option<E>>, G: FnMut() -> F>(
//      repeater: F,
//  ) -> Repeat<G>;
//  /// Create an event iterator, endlessly repeating the same future, using the
//  /// output as the event.
//  pub fn repeat<E, F: Future<Output = E> + Clone>(event: impl F)
//      -> Repeat<F>;
//  /// Create an event iterator, endlessly repeating a closure which provides
//  /// the futures, using the output as the event.
//  pub fn repeat_with<F: Future, G: FnMut() -> F>(repeater: G) -> Repeat<G>;
//  /// Create an event iterator, which yields an event exactly once by polling
//  /// the provided future.
//  pub fn once<F: Future>(f: F)
//  /// Create an event iterator that lazily generates a value exactly once by
//  /// invoking the provided closure and polling the returned future.
//  pub fn once_with<F: Future, G: FnOnce() -> F>(gen: G)
