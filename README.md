rustracing
==========

[![Crates.io: rustracing](https://img.shields.io/crates/v/rustracing.svg)](https://crates.io/crates/rustracing)
[![Documentation](https://docs.rs/rustracing/badge.svg)](https://docs.rs/rustracing)
[![Actions Status](https://github.com/sile/rustracing/workflows/CI/badge.svg)](https://github.com/sile/rustracing/actions)
[![Coverage Status](https://coveralls.io/repos/github/sile/rustracing/badge.svg?branch=master)](https://coveralls.io/github/sile/rustracing?branch=master)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[OpenTracing] API for Rust.

[Documentation](https://docs.rs/rustracing)

Examples
--------

```rust
use rustracing::sampler::AllSampler;
use rustracing::tag::Tag;
use rustracing::Tracer;
use std::thread;
use std::time::Duration;

// Creates a tracer
let (span_tx, span_rx) = crossbeam_channel::bounded(10);
let tracer = Tracer::with_sender(AllSampler, span_tx);
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
while let Ok(span) = span_rx.try_recv() {
    println!("# SPAN: {:?}", span);
}
```

As an actual usage example of the crate and an implementation of the [OpenTracing] API,
it may be helpful to looking at [rustracing_jaeger] crate.

References
----------

- [The OpenTracing Semantic Specification (v1.1)][specification]

[OpenTracing]: http://opentracing.io/
[specification]: https://github.com/opentracing/specification/blob/master/specification.md
[rustracing_jaeger]: https://github.com/sile/rustracing_jaeger
