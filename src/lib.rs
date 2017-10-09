//! OpenTracing API for Rust
//!
//! # References
//!
//! - [The OpenTracing Semantic Specification (v1.1)][specification]
//!
//! [specification]: https://github.com/opentracing/specification/blob/master/specification.md

pub use trace::{Tracer, SpanReceiver};
pub use span::Span;

pub mod carrier;
pub mod convert;
pub mod span;
pub mod tag;
pub mod trace;
