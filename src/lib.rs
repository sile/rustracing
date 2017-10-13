//! OpenTracing API for Rust
//!
//! # References
//!
//! - [The OpenTracing Semantic Specification (v1.1)][specification]
//!
//! [specification]: https://github.com/opentracing/specification/blob/master/specification.md
extern crate backtrace;

pub use trace::{Tracer, Sampler, NoopTracer, DiscardSampler, AlwaysSampler, SpanOptions};
pub use span::Span;

pub mod carrier;
pub mod convert;
pub mod log;
pub mod span;
pub mod tag;

mod trace;
