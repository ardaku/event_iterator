//! #### Asynchronous lending iterator
//!
//! A modified `AsyncIterator` or `Stream` using GATs to enable lending.
//!
//! # Getting Started
//!
//! The following example shows how to implement and use an event iterator to
//! print to stdout:
//!
//! ```console
//! Hello, world!
//! Hello, again!
//! ```
//!
//! ## Example
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

mod as_event_iter;
mod enumerate;
mod event_iter;
mod filter;
mod filter_map;
mod from_iter;
mod fuse;
mod inspect;
mod map;
mod next;
mod take;
mod take_while;
mod tear;

pub use self::{
    as_event_iter::{AsEventIter, AsEventIterator},
    enumerate::Enumerate,
    event_iter::EventIterator,
    filter::Filter,
    filter_map::FilterMap,
    from_iter::{from_iter, FromIter},
    fuse::Fuse,
    inspect::Inspect,
    map::Map,
    next::Next,
    take::Take,
    take_while::TakeWhile,
    tear::Tear,
};

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
