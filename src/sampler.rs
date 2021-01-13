//! `Sampler` trait and its built-in implementations.
use crate::span::CandidateSpan;
use crate::{ErrorKind, Result};
use rand::{self, Rng};

/// `Sampler` decides whether a new trace should be sampled or not.
pub trait Sampler<T> {
    /// This method decides whether a trace with given `span` should be sampled.
    fn is_sampled(&self, span: &CandidateSpan<T>) -> bool;

    /// Returns the sampler that samples a trace if `self` or `other` decides to sample it.
    fn or<U>(self, other: U) -> OrSampler<Self, U>
    where
        Self: Sized,
        U: Sampler<T>,
    {
        OrSampler(self, other)
    }

    /// Returns the sampler that samples a trace if both of `self` and `other` decides to sample it.
    fn and<U>(self, other: U) -> AndSampler<Self, U>
    where
        Self: Sized,
        U: Sampler<T>,
    {
        AndSampler(self, other)
    }

    /// Converts into `BoxSampler`.
    fn boxed(self) -> BoxSampler<T>
    where
        Self: Sized + Send + Sync + 'static,
    {
        Box::new(self)
    }
}
impl<T> Sampler<T> for BoxSampler<T> {
    fn is_sampled(&self, span: &CandidateSpan<T>) -> bool {
        (**self).is_sampled(span)
    }
    fn boxed(self) -> BoxSampler<T>
    where
        Self: Sized + Send + 'static,
    {
        self
    }
}

/// Boxed version of `Sampler`.
pub type BoxSampler<T> = Box<dyn Sampler<T> + Send + Sync + 'static>;

/// This samples a certain percentage of traces.
#[derive(Debug, Clone)]
pub struct ProbabilisticSampler {
    sampling_rate: f64,
}
impl ProbabilisticSampler {
    /// Makes a new `ProbabilisticSampler` instance.
    ///
    /// # Errors
    ///
    /// If `sampling_rate` is not in the range `0.0...1.0`,
    /// it will return an error with the kind `ErrorKind::InvalidInput`.
    pub fn new(sampling_rate: f64) -> Result<Self> {
        track_assert!(0.0 <= sampling_rate, ErrorKind::InvalidInput);
        track_assert!(sampling_rate <= 1.0, ErrorKind::InvalidInput);
        Ok(ProbabilisticSampler { sampling_rate })
    }
}
impl<T> Sampler<T> for ProbabilisticSampler {
    fn is_sampled(&self, _span: &CandidateSpan<T>) -> bool {
        rand::thread_rng().gen_range(0.0..1.0) < self.sampling_rate
    }
}

/// This samples traces which have one or more references.
#[derive(Debug, Clone)]
pub struct PassiveSampler;
impl<T> Sampler<T> for PassiveSampler {
    fn is_sampled(&self, span: &CandidateSpan<T>) -> bool {
        !span.references().is_empty()
    }
}

/// This samples no traces.
#[derive(Debug, Clone)]
pub struct NullSampler;
impl<T> Sampler<T> for NullSampler {
    fn is_sampled(&self, _span: &CandidateSpan<T>) -> bool {
        false
    }
}

/// This samples all traces.
#[derive(Debug, Clone)]
pub struct AllSampler;
impl<T> Sampler<T> for AllSampler {
    fn is_sampled(&self, _span: &CandidateSpan<T>) -> bool {
        true
    }
}

/// `or` combinator.
#[derive(Debug, Clone)]
pub struct OrSampler<A, B>(A, B);
impl<A, B, T> Sampler<T> for OrSampler<A, B>
where
    A: Sampler<T>,
    B: Sampler<T>,
{
    fn is_sampled(&self, span: &CandidateSpan<T>) -> bool {
        self.0.is_sampled(span) || self.1.is_sampled(span)
    }
}

/// `and` combinator.
#[derive(Debug, Clone)]
pub struct AndSampler<A, B>(A, B);
impl<A, B, T> Sampler<T> for AndSampler<A, B>
where
    A: Sampler<T>,
    B: Sampler<T>,
{
    fn is_sampled(&self, span: &CandidateSpan<T>) -> bool {
        self.0.is_sampled(span) && self.1.is_sampled(span)
    }
}
