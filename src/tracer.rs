use std::borrow::Cow;
use std::time::SystemTime;

use span::{Span, SpanContext, Reference};
use tag::Tag;

#[derive(Debug)]
pub struct TracerBuilder {
    operation_name: Cow<'static, str>,
}
impl TracerBuilder {
    pub fn new<T: Into<Cow<'static, str>>>(operation_name: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        TracerBuilder { operation_name: operation_name.into() }
    }
}

// TODO: s/trait/struct/
pub trait Tracer {
    fn start_span(
        &mut self,
        operation_name: &str,
        references: Vec<Reference>,
        start_time: Option<SystemTime>,
        tags: Vec<Tag>,
    ) -> Span;

    // TODO: Add `format`
    fn inject_context<C>(&mut self, context: &SpanContext, carrier: &mut C) -> Result<(), C::Error>
    where
        C: Carrier;

    // TODO: Add `format`
    fn extract_context<C>(&mut self, carrier: &mut C) -> Result<(), C::Error>
    where
        C: Carrier;
}

#[derive(Debug)]
pub struct NoopTracer;

pub trait Carrier {
    type Error;
}

#[derive(Debug)]
pub enum Format {
    TextMap,
    HttpHeaders,
    Binary,
}
