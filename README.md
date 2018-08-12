rustracing
==========

[![Crates.io: rustracing](http://meritbadge.herokuapp.com/rustracing)](https://crates.io/crates/rustracing)
[![Documentation](https://docs.rs/rustracing/badge.svg)](https://docs.rs/rustracing)
[![Build Status](https://travis-ci.org/sile/rustracing.svg?branch=master)](https://travis-ci.org/sile/rustracing)
[![Code Coverage](https://codecov.io/gh/sile/rustracing/branch/master/graph/badge.svg)](https://codecov.io/gh/sile/rustracing/branch/master)
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
while let Ok(span) = span_rx.try_recv() {
    println!("# SPAN: {:?}", span);
}
```

As an actual usage example of the crate and an implmentation of the [OpenTracing] API,
it may be helpful to looking at [rustracing_jaeger] crate.

References
----------

- [The OpenTracing Semantic Specification (v1.1)][specification]

[OpenTracing]: http://opentracing.io/
[specification]: https://github.com/opentracing/specification/blob/master/specification.md
[rustracing_jaeger]: https://github.com/sile/rustracing_jaeger
