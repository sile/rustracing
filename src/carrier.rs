use std::error::Error;

use span::SpanContext;

// TODO: struct NeverFails

pub trait Inject<T> {
    type Error: Error;
    fn inject(&mut self, context: &SpanContext<T>) -> Result<(), Self::Error>;
}

pub trait Extract<T> {
    type Error: Error;
    fn extract(&mut self) -> Result<Option<SpanContext<T>>, Self::Error>;
}
