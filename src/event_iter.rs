use core::{
    ops::Deref,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{
    Enumerate, Filter, FilterMap, Fuse, Inspect, Map, Next, Take, TakeWhile,
    Tear,
};

/// Asynchronous lending iterator
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
    ///
    /// # Example
    ///
    /// ```rust
    #[doc = include_str!("../examples/next.rs")]
    /// ```
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
    ///
    /// # Example
    ///
    /// ```rust
    #[doc = include_str!("../examples/next_unpinned.rs")]
    /// ```
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
    ///
    /// # Example
    ///
    /// ```rust
    #[doc = include_str!("../examples/size_hint.rs")]
    /// ```
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    /// Take a closure and create an event iterator which calls that closure on
    /// each event.
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
    /// `map()` is conceptually similar to a `while let Some(_) = _.await` loop.
    /// However, as `map()` is lazy, it is best used when you’re already working
    /// with other event iterators.  If you’re doing some sort of looping for a
    /// side effect, it’s considered more idiomatic to use
    /// `while let Some(_) = _.await` than `map()`.
    ///
    /// # Example
    ///
    /// ```rust
    #[doc = include_str!("../examples/map.rs")]
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
    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: for<'me> FnMut(Self::Event<'me>) -> B,
    {
        Map::new(self, f)
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
        P: for<'me> FnMut(&Self::Event<'me>) -> bool,
    {
        Filter::new(self, predicate)
    }

    /// Create an event iterator that both filters and maps.
    ///
    /// The returned event iterator yields only the events for which the
    /// supplied closure returns `Some(event)`.
    ///
    /// `filter_map()` can be used to make chains of [`filter()`](Self::filter)
    /// and [`map()`](Self::map) more concise.  The example below shows how a
    /// `map().filter().map()` can be shortened to a single call to
    /// `filter_map()`.
    ///
    /// # Example
    ///
    /// ```rust
    #[doc = include_str!("../examples/filter_map.rs")]
    /// ```
    fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>
    where
        Self: Sized,
        F: for<'me> FnMut(Self::Event<'me>) -> Option<B>,
    {
        FilterMap::new(self, f)
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
        F: for<'me> FnMut(&Self::Event<'me>),
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
        P: for<'me> FnMut(&Self::Event<'me>) -> bool,
    {
        TakeWhile::new(self, predicate)
    }
}

impl<T> EventIterator for T
where
    T: Deref + ?Sized,
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
