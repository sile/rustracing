//! [OpenTracing][opentracing] API for Rust
//!
//! # Examples
//!
//! ```
//! # extern crate rustracing;
//! use rustracing::sampler::AllSampler;
//! use rustracing::tag::Tag;
//! use rustracing::Tracer;
//! use std::thread;
//! use std::time::Duration;
//!
//! # fn main() {
//! // Creates a tracer
//! let (tracer, span_rx) = Tracer::new(AllSampler);
//! {
//!     // Starts "parent" span
//!     let parent_span = tracer.span("parent").start_with_state(());
//!     thread::sleep(Duration::from_millis(10));
//!     {
//!         // Starts "child" span
//!         let mut child_span = tracer
//!             .span("child_span")
//!             .child_of(&parent_span)
//!             .tag(Tag::new("key", "value"))
//!             .start_with_state(());
//!
//!         child_span.log(|log| {
//!             log.error().message("a log message");
//!         });
//!     } // The "child" span dropped and will be sent to `span_rx`
//! } // The "parent" span dropped and will be sent to `span_rx`
//!
//! // Outputs finished spans to the standard output
//! while let Ok(span) = span_rx.try_recv() {
//!     println!("# SPAN: {:?}", span);
//! }
//! # }
//! ```
//!
//! As an actual usage example of the crate and an implmentation of the [OpenTracing] API,
//! it may be helpful to looking at [rustracing_jaeger] crate.
//!
//! # References
//!
//! - [The OpenTracing Semantic Specification (v1.1)][specification]
//!
//! [opentracing]: http://opentracing.io/
//! [specification]: https://github.com/opentracing/specification/blob/master/specification.md
//! [rustracing_jaeger]: https://github.com/sile/rustracing_jaeger
#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(doc_markdown))]
#[cfg(feature = "stacktrace")]
extern crate backtrace;
extern crate rand;
#[macro_use]
extern crate trackable;

pub use error::{Error, ErrorKind};
pub use tracer::Tracer;

pub mod carrier;
pub mod convert;
pub mod log;
pub mod sampler;
pub mod span;
pub mod tag;

mod error;
mod tracer;

/// This crate specific `Result` type.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use super::*;
    use sampler::AllSampler;
    use tag::{StdTag, Tag};

    #[test]
    fn it_works() {
        let (tracer, span_rx) = Tracer::new(AllSampler);
        {
            let span = tracer.span("it_works").start_with_state(());
            let mut child = span.child("child", |options| options.start_with_state(()));
            child.set_tags(|| StdTag::peer_addr("127.0.0.1:80".parse().unwrap()));
        }

        let span = span_rx.try_recv().unwrap();
        assert_eq!(span.operation_name(), "child");

        let span = span_rx.try_recv().unwrap();
        assert_eq!(span.operation_name(), "it_works");
    }

    #[test]
    fn example_code_works() {
        // Creates a tracer
        let (tracer, span_rx) = Tracer::new(AllSampler);
        {
            // Starts "parent" span
            let parent_span = tracer.span("parent").start_with_state(());
            thread::sleep(Duration::from_millis(10));
            {
                // Starts "child" span
                let mut child_span = tracer
                    .span("child_span")
                    .child_of(&parent_span)
                    .tag(Tag::new("key", "value"))
                    .start_with_state(());

                child_span.log(|log| {
                    log.error().message("a log message");
                });
            } // The "child" span dropped and will be sent to `span_rx`
        } // The "parent" span dropped and will be sent to `span_rx`

        // Outputs finished spans to the standard output
        let mut count = 0;
        while let Ok(span) = span_rx.try_recv() {
            println!("# SPAN: {:?}", span);
            count += 1;
        }
        assert_eq!(count, 2);
    }
}
