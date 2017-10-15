//! [OpenTracing][opentracing] API for Rust
//!
//! # References
//!
//! - [The OpenTracing Semantic Specification (v1.1)][specification]
//!
//! [opentracing]: http://opentracing.io/
//! [specification]: https://github.com/opentracing/specification/blob/master/specification.md
#![warn(missing_docs)]
extern crate backtrace;
#[macro_use]
extern crate trackable;

pub use error::{Error, ErrorKind};
pub use trace::{Tracer, Sampler, NoopTracer, DiscardSampler, AlwaysSampler, SpanOptions};
pub use span::Span;

pub mod carrier;
pub mod convert;
pub mod log;
pub mod span;
pub mod tag;

mod error;
mod trace;

pub type Result<T> = std::result::Result<T, Error>;
