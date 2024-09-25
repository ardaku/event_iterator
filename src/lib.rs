//! #### Asynchronous lending iterator
//!
//! A modified `AsyncIterator` or `Stream` using GATs to enable lending.
//!
//! The primary trait provided by this crate is [`EventIterator`].
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
#![doc(
    html_logo_url = "https://ardaku.github.io/mm/logo.svg",
    html_favicon_url = "https://ardaku.github.io/mm/icon.svg",
    html_root_url = "https://docs.rs/event_iterator"
)]
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
mod empty;
mod enumerate;
mod event_iter;
mod filter;
mod filter_map;
mod from_fn;
mod from_iter;
mod fuse;
mod inspect;
mod map;
mod next;
mod once;
mod once_with;
mod pending;
mod poll_fn;
mod ready;
mod repeat;
mod repeat_with;
mod take;
mod take_while;
mod tear;

pub use self::{
    as_event_iter::{AsEventIter, AsEventIterator},
    empty::{empty, Empty},
    enumerate::Enumerate,
    event_iter::EventIterator,
    filter::Filter,
    filter_map::FilterMap,
    from_fn::{from_fn, FromFn},
    from_iter::{from_iter, FromIter},
    fuse::Fuse,
    inspect::Inspect,
    map::Map,
    next::Next,
    once::{once, Once},
    once_with::{once_with, OnceWith},
    pending::{pending, Pending},
    poll_fn::{poll_fn, PollFn},
    ready::{ready, Ready},
    repeat::{repeat, Repeat},
    repeat_with::{repeat_with, RepeatWith},
    take::Take,
    take_while::TakeWhile,
    tear::Tear,
};
