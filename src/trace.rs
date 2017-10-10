use std::borrow::Cow;
use std::sync::mpsc;

use span::{SpanBuilder, FinishedSpan, InactiveSpan, Span, SpanReceiver};

#[derive(Debug)]
pub struct Tracer<S, T> {
    sampler: S,
    span_tx: mpsc::Sender<FinishedSpan<T>>,
}
impl<S: Sampler, T> Tracer<S, T> {
    pub fn new(sampler: S) -> (Self, SpanReceiver<T>) {
        let (tx, rx) = mpsc::channel();
        (
            Tracer {
                sampler,
                span_tx: tx,
            },
            rx,
        )
    }
    pub fn clone_with_sampler<U: Sampler>(&self, sampler: U) -> Tracer<U, T> {
        Tracer {
            sampler,
            span_tx: self.span_tx.clone(),
        }
    }
    pub fn start_span<F, OperationName>(&mut self, f: F) -> Span<T>
    where
        F: FnOnce(&mut SpanBuilder<T>) -> Option<(OperationName, T)>,
        OperationName: Into<Cow<'static, str>>,
    {
        if !self.sampler.preselect() {
            Span::disabled()
        } else {
            let mut builder = SpanBuilder::new();
            if let Some((operaion_name, state)) = f(&mut builder) {
                let (span, references) = builder.finish(operaion_name);
                if self.sampler.select(&span) {
                    span.activate(state, references, self.span_tx.clone())
                } else {
                    Span::disabled()
                }
            } else {
                Span::disabled()
            }
        }
    }
}
impl<S: Clone, T> Clone for Tracer<S, T> {
    fn clone(&self) -> Self {
        Tracer {
            sampler: self.sampler.clone(),
            span_tx: self.span_tx.clone(),
        }
    }
}

pub trait Sampler {
    fn preselect(&mut self) -> bool;
    fn select(&mut self, span: &InactiveSpan) -> bool;
}

#[derive(Debug, Clone)]
pub struct DiscardSampler;
impl Sampler for DiscardSampler {
    fn preselect(&mut self) -> bool {
        false
    }
    fn select(&mut self, _span: &InactiveSpan) -> bool {
        false
    }
}

// TODO: name
#[derive(Debug, Clone)]
pub struct DebugSampler;
impl Sampler for DebugSampler {
    fn preselect(&mut self) -> bool {
        true
    }
    fn select(&mut self, _span: &InactiveSpan) -> bool {
        true
    }
}

// TODO: ProbabilisticSampler

pub type NoopTracer<T> = Tracer<DiscardSampler, T>;
