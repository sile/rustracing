use std::borrow::Cow;
use std::sync::Arc;
use std::sync::mpsc;
use std::time::SystemTime;

use sampler::Sampler;
use span::{FinishedSpan, Span, SpanReceiver, SpanReference, BaggageItem, SpanContext};
use tag::Tag;
use convert::MaybeAsRef;

// TODO: name
#[derive(Debug)]
pub struct SpanOptions<'a, T: 'a> {
    tags: &'a [Tag],
    references: &'a [SpanReference<T>],
    baggage_items: &'a [BaggageItem],
}
impl<'a, T: 'a> SpanOptions<'a, T> {
    pub fn tags(&self) -> &[Tag] {
        self.tags
    }
    pub fn references(&self) -> &[SpanReference<T>] {
        self.references
    }
    pub fn baggage_items(&self) -> &[BaggageItem] {
        self.baggage_items
    }
}

#[derive(Debug)]
pub struct StartSpanOptions<'a, S: 'a, T: 'a> {
    tracer: &'a Tracer<S, T>,
    operation_name: Cow<'static, str>,
    start_time: Option<SystemTime>,
    tags: Vec<Tag>,
    references: Vec<SpanReference<T>>,
    baggage_items: Vec<BaggageItem>,
}
impl<'a, S: 'a, T: 'a> StartSpanOptions<'a, S, T>
where
    S: Sampler<T>,
{
    fn new<N>(tracer: &'a Tracer<S, T>, operation_name: N) -> Self
    where
        N: Into<Cow<'static, str>>,
    {
        StartSpanOptions {
            tracer,
            operation_name: operation_name.into(),
            start_time: None,
            tags: Vec::new(),
            references: Vec::new(),
            baggage_items: Vec::new(),
        }
    }
    pub fn start_time(mut self, time: SystemTime) -> Self {
        self.start_time = Some(time);
        self
    }
    pub fn tag(mut self, tag: Tag) -> Self {
        self.tags.push(tag);
        self
    }
    pub fn child_of<C>(mut self, context: C) -> Self
    where
        C: MaybeAsRef<SpanContext<T>>,
        T: Clone,
    {
        if let Some(context) = context.maybe_as_ref() {
            let reference = SpanReference::ChildOf(context.state().clone());
            self.references.push(reference);
            self.baggage_items.extend(
                context.baggage_items().iter().cloned(),
            );
        }
        self
    }
    pub fn follows_from<C>(mut self, context: &C) -> Self
    where
        C: MaybeAsRef<SpanContext<T>>,
        T: Clone,
    {
        if let Some(context) = context.maybe_as_ref() {
            let reference = SpanReference::FollowsFrom(context.state().clone());
            self.references.push(reference);
            self.baggage_items.extend(
                context.baggage_items().iter().cloned(),
            );
        }
        self
    }
    fn options(&self) -> SpanOptions<T> {
        SpanOptions {
            references: &self.references,
            tags: &self.tags,
            baggage_items: &self.baggage_items,
        }
    }
    pub fn start(mut self) -> Span<T>
    where
        T: for<'b> From<SpanOptions<'b, T>>,
    {
        self.tags.reverse();
        self.tags.sort_by(|a, b| a.name().cmp(b.name()));
        self.tags.dedup_by(|a, b| a.name() == b.name());
        self.baggage_items.reverse();

        if !self.tracer.sampler.is_sampled(&self.options()) {
            return Span::disabled();
        }

        let state = T::from(self.options());
        let span = ::span::InactiveSpan {
            operation_name: self.operation_name,
            start_time: self.start_time.unwrap_or_else(|| SystemTime::now()),
            tags: self.tags,
            references: self.references.len(),
            baggage_items: self.baggage_items,
        };
        span.activate(state, self.references, self.tracer.span_tx.clone())
    }
    // TODO: F
    pub fn start_with_state(mut self, state: T) -> Span<T> {
        self.tags.reverse();
        self.tags.sort_by(|a, b| a.name().cmp(b.name()));
        self.tags.dedup_by(|a, b| a.name() == b.name());
        self.baggage_items.reverse();

        if !self.tracer.sampler.is_sampled(&self.options()) {
            return Span::disabled();
        }

        let span = ::span::InactiveSpan {
            operation_name: self.operation_name,
            start_time: self.start_time.unwrap_or_else(|| SystemTime::now()),
            tags: self.tags,
            references: self.references.len(),
            baggage_items: self.baggage_items,
        };
        span.activate(state, self.references, self.tracer.span_tx.clone())
    }
}

#[derive(Debug)]
pub struct Tracer<S, T> {
    pub(crate) sampler: Arc<S>, // TODO: private
    pub(crate) span_tx: mpsc::Sender<FinishedSpan<T>>,
}
impl<S: Sampler<T>, T> Tracer<S, T> {
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
    pub fn clone_with_sampler<U: Sampler<T>>(&self, sampler: U) -> Tracer<U, T> {
        Tracer {
            sampler: Arc::new(sampler),
            span_tx: self.span_tx.clone(),
        }
    }
    pub fn span<N>(&self, operation_name: N) -> StartSpanOptions<S, T>
    where
        N: Into<Cow<'static, str>>,
    {
        StartSpanOptions::new(self, operation_name)
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
