use std::error::Error;
use std::fmt;

use span::SpanContext;

#[derive(Debug)]
pub struct Never(());
impl Error for Never {
    fn description(&self) -> &str {
        unreachable!()
    }
}
impl fmt::Display for Never {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unreachable!()
    }
}

pub trait Inject<T> {
    type Error: Error;
    fn inject(&mut self, context: &SpanContext<T>) -> Result<(), Self::Error>;
}

pub trait Extract<T> {
    type Error: Error;
    fn extract(&mut self) -> Result<Option<SpanContext<T>>, Self::Error>;
}
