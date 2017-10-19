use std::borrow::Cow;
use std::sync::Arc;
use std::sync::mpsc;

use sampler::Sampler;
use span::{StartSpanOptions, FinishedSpan, SpanReceiver};

/// Tracer.
///
/// # Examples
///
/// ```
/// use rustracing::Tracer;
/// use rustracing::sampler::AllSampler;
///
/// let (tracer, span_rx) = Tracer::new(AllSampler);
/// {
///    let _span = tracer.span("foo").start_with_context(());
/// }
/// let span = span_rx.try_recv().unwrap();
/// assert_eq!(span.operation_name(), "foo");
/// ```
#[derive(Debug)]
pub struct Tracer<S, T> {
    sampler: Arc<S>,
    span_tx: mpsc::Sender<FinishedSpan<T>>,
}
impl<S: Sampler<T>, T> Tracer<S, T> {
    /// Makes a new `Tracer` instance.
    pub fn new(sampler: S) -> (Self, SpanReceiver<T>) {
        let (tx, rx) = mpsc::channel();
        (
            Tracer {
                sampler: Arc::new(sampler),
                span_tx: tx,
            },
            rx,
        )
    }

    /// Returns `StartSpanOptions` for starting a span which has the name `operation_name`.
    pub fn span<N>(&self, operation_name: N) -> StartSpanOptions<S, T>
    where
        N: Into<Cow<'static, str>>,
    {
        StartSpanOptions::new(self, operation_name)
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

    pub(crate) fn sampler(&self) -> &S {
        &self.sampler
    }
    pub(crate) fn span_tx(&self) -> mpsc::Sender<FinishedSpan<T>> {
        self.span_tx.clone()
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
