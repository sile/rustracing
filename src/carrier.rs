use span::SpanContext;

pub trait Carrier<T> {
    fn inject(&mut self, context: &SpanContext<T>);
    fn extract(&mut self) -> Option<SpanContext<T>>;
}
