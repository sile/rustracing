use std::borrow::Cow;
use std::sync::Arc;
use std::sync::mpsc;

use sampler::Sampler;
use span::{StartSpanOptions, FinishedSpan, SpanReceiver};

/// Tracer.
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

    /// Clone with the given `sampler`.
    pub fn clone_with_sampler<U: Sampler<T>>(&self, sampler: U) -> Tracer<U, T> {
        Tracer {
            sampler: Arc::new(sampler),
            span_tx: self.span_tx.clone(),
        }
    }

    /// Returns `StartSpanOptions` for starting a span which has the name `operation_name`.
    pub fn span<N>(&self, operation_name: N) -> StartSpanOptions<S, T>
    where
        N: Into<Cow<'static, str>>,
    {
        StartSpanOptions::new(self, operation_name)
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
            sampler: self.sampler.clone(),
            span_tx: self.span_tx.clone(),
        }
    }
}
