use crate::sampler::Sampler;
use crate::span::{DefaultSpanReceiver, DefaultSpanSender, SpanSend, StartSpanOptions};
use std::borrow::Cow;
use std::marker::PhantomData;
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
pub struct Tracer<S, T, Sender = DefaultSpanSender<T>> {
    sampler: Arc<S>,
    span_tx: Sender,
    _span_state: PhantomData<T>,
}
impl<S, T> Tracer<S, T, DefaultSpanSender<T>>
where
    S: Sampler<T>,
{
    /// This constructor is mainly for backward compatibility, it has the same interface
    /// as in previous versions except the type of `SpanReceiver`.
    /// It builds an unbounded channel which may cause memory issues if there is no reader,
    /// prefer `with_sender()` alternative with a bounded one.
    pub fn new(sampler: S) -> (Self, DefaultSpanReceiver<T>) {
        let (span_tx, span_rx) = crossbeam_channel::unbounded();
        (Self::with_sender(sampler, span_tx), span_rx)
    }
}
impl<S, T, Sender> Tracer<S, T, Sender>
where
    S: Sampler<T>,
    Sender: SpanSend<T>,
{
    /// Makes a new `Tracer` instance.
    pub fn with_sender(sampler: S, span_tx: Sender) -> Self {
        Tracer {
            sampler: Arc::new(sampler),
            span_tx,
            _span_state: PhantomData,
        }
    }

    /// Returns `StartSpanOptions` for starting a span which has the name `operation_name`.
    pub fn span<N>(&self, operation_name: N) -> StartSpanOptions<S, T, Sender>
    where
        N: Into<Cow<'static, str>>,
    {
        StartSpanOptions::new(operation_name, &self.span_tx, &self.sampler)
    }
}
impl<S, T, Sender> Tracer<S, T, Sender>
where
    Sender: SpanSend<T>,
{
    /// Clone with the given `sampler`.
    pub fn clone_with_sampler<U: Sampler<T>>(&self, sampler: U) -> Tracer<U, T, Sender> {
        Tracer {
            sampler: Arc::new(sampler),
            span_tx: self.span_tx.clone(),
            _span_state: PhantomData,
        }
    }
}
impl<S, T, Sender> Clone for Tracer<S, T, Sender>
where
    Sender: SpanSend<T>,
{
    fn clone(&self) -> Self {
        Tracer {
            sampler: Arc::clone(&self.sampler),
            span_tx: self.span_tx.clone(),
            _span_state: PhantomData,
        }
    }
}
