use core::{
    ops::{Deref, DerefMut},
    pin::Pin,
    task::{Context, Poll},
};

use crate::{
    Enumerate, Filter, FilterMapRef, Fuse, Inspect, MapRef, Next, Take,
    TakeWhile, Tear,
};

/// Asynchronous lending iterator
///
/// Rather than have a single `poll_next()` method as in `Stream` /
/// `AsyncIterator`, event iterators have separate `poll()` and `event()`
/// methods for polling and lending.  Why?  A lot of asynchronous implementation
/// patterns require usage of [`Pin::as_mut()`].  When GATs are in the mix, this
/// usage is impossible since it reduces the lifetime on your pinned reference
/// to `Self`, which would be insufficient for a returned
/// `Poll<Option<Event<'a>>>` lifetime.
///
/// # Example
///
/// ```rust
#[doc = include_str!("../examples/stdin.rs")]
/// ```
pub trait EventIterator {
    /// The type of the events being iterated over
    type Event<'me>: Copy
    where
        Self: 'me;

    /// Attempt to poll the next event of this event iterator, registering the
    /// current task for wakeup if the event is not yet available.
    ///
    /// # Return value
    ///
    /// - `Poll::Pending` means that this event iterator’s next value is not
    ///   ready yet.  Implementations will ensure that the current task will be
    ///   notified when the next value may be ready.
    /// - `Poll::Ready(())` means that the event iterator is either ready to
    ///   lend an event or has terminated.  `event()` should be called to check.
    ///
    /// # Panics
    ///
    /// Once an event iterator has finished (returned `Ready` from `poll()` with
    /// `event()` returning `None`), calling its `poll()` method again may
    /// panic, block forever, or cause other kinds of problems; the
    /// `EventIterator` trait places no requirements on the effects of such a
    /// call.  However, as the `poll()` method is not marked unsafe, Rust’s
    /// usual rules apply: calls must never cause undefined behavior (memory
    /// corruption, incorrect use of unsafe functions, or the like), regardless
    /// of the event iterator’s state.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()>;

    /// Attempt to borrow the current event, and return `None` if the event
    /// iterator is exhausted.
    ///
    /// # Panics
    ///
    /// Calling `event()` before `poll()` may panic, block forever or cause
    /// other kinds of problems; the `EventIterator` trait places no
    /// requirements on the effects of such a call.  However, as the
    /// `poll()` method is not marked unsafe, Rust’s usual rules apply: calls
    /// must never cause undefined behavior (memory corruption, incorrect use of
    /// unsafe functions, or the like), regardless of the event iterator’s
    /// state.
    fn event<'a>(self: Pin<&'a mut Self>) -> Option<Self::Event<'a>>;

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
    ///
    /// # Example
    ///
    /// ```rust
    #[doc = include_str!("../examples/size_hint.rs")]
    /// ```
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    /// Create a future that resolves to the next event in the event iterator.
    ///
    /// This is less flexible than [`next()`](Self::next), but avoids the need
    /// to handle pinning yourself.
    ///
    /// # Example
    ///
    /// ```rust
    #[doc = include_str!("../examples/next.rs")]
    /// ```
    fn next(&mut self) -> Next<'_, Self>
    where
        Self: Sized + Unpin,
    {
        Pin::new(self).next_pinned()
    }

    /// Create a future that resolves to the next event in the event iterator.
    ///
    /// This is more flexible than [`next()`](Self::next), but often more
    /// verbose than needed.
    ///
    /// # Example
    ///
    /// ```rust
    #[doc = include_str!("../examples/next_pinned.rs")]
    /// ```
    fn next_pinned<'a>(self: Pin<&'a mut Self>) -> Next<'a, Self>
    where
        Self: Sized,
    {
        Next::new(self)
    }

    /// Take a closure and create an event iterator of references which calls
    /// that closure on each event.
    ///
    /// `map_ref()` transforms one event iterator into another, by means of its
    /// argument: something that implements [`FnMut`].  It produces a new event
    /// iterator which calls this closure on each event of the original event
    /// iterator.
    ///
    /// If you are good at thinking in types, you can think of `map_ref()` like
    /// this: If you have an iterator that gives you elements of some type `A`,
    /// and you want an iterator of some other type `B`, you can use
    /// `map_ref()`, passing a closure that takes an `A` and returns a `B`.
    ///
    /// `map_ref()` is conceptually similar to a `while let Some(_) = _.await`
    /// loop.  However, as `map_ref()` is lazy, it is best used when you’re
    /// already working with other event iterators.  If you’re doing some sort
    /// of looping for a side effect, it’s considered more idiomatic to use
    /// `while let Some(_) = _.await` than `map_ref()`.
    ///
    /// # Example
    ///
    /// ```rust
    #[doc = include_str!("../examples/map_ref.rs")]
    /// ```
    /// 
    /// Output:
    /// ```console
    /// uwu
    /// uwuuwu
    /// uwuuwuuwu
    /// uwuuwuuwuuwu
    /// uwuuwuuwuuwuuwu
    /// ```
    fn map_ref<E, F>(self, f: F) -> MapRef<Self, F, E>
    where
        Self: Sized,
        F: for<'me> FnMut(Self::Event<'me>) -> E,
    {
        MapRef::new(self, f)
    }

    /// Create an event iterator which uses a closure to determine if an event
    /// should be yielded.
    ///
    /// Given an event the closure must return `true` or `false`.  The returned
    /// event iterator will yield only the events for which the closure returns
    /// `true`.
    ///
    /// # Example
    ///
    /// ```rust
    #[doc = include_str!("../examples/filter.rs")]
    /// ```
    fn filter<P>(self, predicate: P) -> Filter<Self, P>
    where
        Self: Sized,
        P: for<'me> FnMut(Self::Event<'me>) -> bool,
    {
        Filter::new(self, predicate)
    }

    /// Create an event iterator that both filters and maps.
    ///
    /// The returned event iterator yields only the events for which the
    /// supplied closure returns `Some(event)`.
    ///
    /// `filter_map_ref()` can be used to make chains of
    /// [`filter()`](Self::filter) and [`map_ref()`](Self::map_ref) more
    /// concise.  The example below shows how a `map_ref().filter().map_ref()`
    /// can be shortened to a single call to `filter_map_ref()`.
    ///
    /// # Example
    ///
    /// ```rust
    #[doc = include_str!("../examples/filter_map_ref.rs")]
    /// ```
    fn filter_map_ref<E, F>(self, f: F) -> FilterMapRef<Self, F, E>
    where
        Self: Sized,
        F: for<'me> FnMut(Self::Event<'me>) -> Option<E>,
    {
        FilterMapRef::new(self, f)
    }

    /// Do something with each event of an event iterator, passing the value on.
    ///
    /// It’s more common for `inspect()` to be used as a debugging tool than to
    /// exist in your final code, but applications may find it useful in certain
    /// situations when errors need to be logged before being discarded.
    ///
    /// # Example
    ///
    /// ```rust
    #[doc = include_str!("../examples/inspect.rs")]
    /// ```
    /// 
    /// Output:
    /// ```console
    /// uwu
    /// uwuuwu
    /// uwuuwuuwu
    /// uwuuwuuwuuwu
    /// uwuuwuuwuuwuuwu
    /// ```
    fn inspect<F>(self, f: F) -> Inspect<Self, F>
    where
        Self: Sized,
        F: for<'me> FnMut(Self::Event<'me>),
    {
        Inspect::new(self, f)
    }

    /// Create an event iterator which gives the current iteration count as well
    /// as the next event.
    ///
    /// The event iterator returned yields pairs `(i, e)`, where `i` is the
    /// current index of iteration and `e` is the event returned by the event
    /// iterator.
    ///
    /// `enumerate()` keeps its count as a [`usize`].
    ///
    /// # Overflow Behavior
    ///
    /// The method does no guarding against overflows, so enumerating more than
    /// [`usize::MAX`] elements either produces the wrong result or panics.  If
    /// debug assertions are enabled, a panic is guaranteed.
    ///
    /// # Panics
    ///
    /// The returned event iterator might panic if the to-be-returned index
    /// would overflow a [`usize`].
    ///
    /// The returned event iterator might panic if [`EventIterator::event()`] is
    /// called before [`EventIterator::poll()`].
    ///
    /// # Example
    ///
    /// ```rust
    #[doc = include_str!("../examples/enumerate.rs")]
    /// ```
    fn enumerate(self) -> Enumerate<Self>
    where
        Self: Sized,
    {
        Enumerate::new(self)
    }

    /// Create an event iterator which ends after the first `Ready(None)`.
    ///
    /// After an event iterator returns `Ready(None)`, future calls may or may
    /// not yield `Ready(Some(E))` again.  `fuse()` adapts an event iterator,
    /// ensuring that after a `Ready(None)` is returned, it will always return
    /// `Ready(None)` forever.
    ///
    /// If you want to return `Pending` forever instead, use
    /// [`tear()`](EventIterator::tear())
    ///
    /// # Example
    ///
    /// ```rust
    #[doc = include_str!("../examples/fuse.rs")]
    /// ```
    fn fuse(self) -> Fuse<Self>
    where
        Self: Sized,
    {
        Fuse::new(self)
    }

    /// Create an event iterator which ends after the first `Ready(None)`.
    ///
    /// After an event iterator returns `Ready(None)`, future calls may or may
    /// not yield `Ready(Some(E))` again.  `tear()` adapts an event iterator,
    /// ensuring that after a `Ready(None)` is returned, it will always return
    /// `Pending` forever.
    ///
    /// If you want to return `Ready(None)` forever instead, use
    /// [`fuse()`](EventIterator::fuse())
    ///
    /// # Example
    ///
    /// ```rust
    #[doc = include_str!("../examples/tear.rs")]
    /// ```
    fn tear(self) -> Tear<Self>
    where
        Self: Sized,
    {
        Tear::new(self)
    }

    /// Create an event iterator that yields the first `n` events, or fewer if
    /// the underlying event iterator ends sooner.
    ///
    /// `take(n)` yields events until `n` events are yielded or the end of the
    /// event iterator is reached (whichever happens first).  The returned event
    /// iterator is a prefix of length `n` if the original iterator contains at
    /// least `n` events, otherwise it contains all of the (fewer than `n`)
    /// events of the original event iterator.
    ///
    /// # Example
    ///
    /// ```rust
    #[doc = include_str!("../examples/take.rs")]
    /// ```
    fn take(self, n: usize) -> Take<Self>
    where
        Self: Sized,
    {
        Take::new(self, n)
    }

    /// Create an event iterator that yields elements based on a predicate.
    ///
    /// `take_while()` takes a closure as an argument.  It will call this
    /// closure on each event of the event iterator, and yield events while it
    /// returns `true`.
    ///
    /// After `false` is returned, `take_while()`’s job is over, and the rest of
    /// the events are ignored.
    ///
    /// # Example
    ///
    /// ```rust
    #[doc = include_str!("../examples/take_while.rs")]
    /// ```
    fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>
    where
        Self: Sized,
        P: for<'me> FnMut(Self::Event<'me>) -> bool,
    {
        TakeWhile::new(self, predicate)
    }
}

impl<T> EventIterator for T
where
    T: Deref + DerefMut + ?Sized + Unpin,
    T::Target: EventIterator + Unpin,
{
    type Event<'me>
        = <<T as Deref>::Target as EventIterator>::Event<'me>
    where
        Self: 'me;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        Pin::new(&mut **self.get_mut()).poll(cx)
    }

    fn event<'a>(self: Pin<&'a mut Self>) -> Option<Self::Event<'a>> {
        Pin::new(&mut **self.get_mut()).event()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (**self).size_hint()
    }
}
