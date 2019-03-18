use crate::sampler::Sampler;
use crate::span::{SpanReceiver, SpanSender, StartSpanOptions};
use std::borrow::Cow;
use std::sync::Arc;

/// Tracer.
///
/// # Examples
///
/// ```
/// use rustracing::Tracer;
/// use rustracing::sampler::AllSampler;
///
/// let (span_tx, span_rx) = crossbeam_channel::bounded(10);
/// let tracer = Tracer::with_sender(AllSampler, span_tx);
/// {
///    let _span = tracer.span("foo").start_with_state(());
/// }
/// let span = span_rx.try_recv().unwrap();
/// assert_eq!(span.operation_name(), "foo");
/// ```
#[derive(Debug)]
pub struct Tracer<S, T> {
    sampler: Arc<S>,
    span_tx: SpanSender<T>,
}
impl<S: Sampler<T>, T> Tracer<S, T> {
    /// This constructor is mainly for backward compatibility, it has the same interface
    /// as in previous versions except the type of `SpanReceiver`.
    /// It builds an unbounded channel which may cause memory issues if there is no reader,
    /// prefer `with_sender()` alternative with a bounded one.
    pub fn new(sampler: S) -> (Self, SpanReceiver<T>) {
        let (span_tx, span_rx) = crossbeam_channel::unbounded();
        (Self::with_sender(sampler, span_tx), span_rx)
    }

    /// Makes a new `Tracer` instance.
    pub fn with_sender(sampler: S, span_tx: SpanSender<T>) -> Self {
        Tracer {
            sampler: Arc::new(sampler),
            span_tx,
        }
    }

    /// Returns `StartSpanOptions` for starting a span which has the name `operation_name`.
    pub fn span<N>(&self, operation_name: N) -> StartSpanOptions<S, T>
    where
        N: Into<Cow<'static, str>>,
    {
        StartSpanOptions::new(operation_name, &self.span_tx, &self.sampler)
    }
}
impl<S, T> Tracer<S, T> {
    /// Clone with the given `sampler`.
    pub fn clone_with_sampler<U: Sampler<T>>(&self, sampler: U) -> Tracer<U, T> {
        Tracer {
            sampler: Arc::new(sampler),
            span_tx: self.span_tx.clone(),
        }
    }
}
impl<S, T> Clone for Tracer<S, T> {
    fn clone(&self) -> Self {
        Tracer {
            sampler: Arc::clone(&self.sampler),
            span_tx: self.span_tx.clone(),
        }
    }
}
