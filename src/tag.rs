// TODO: `SpanTag`
//
// See: https://github.com/opentracing/specification/blob/master/semantic_conventions.md
// (standard tag)
#[derive(Debug, Clone)]
pub struct Tag {
    pub key: String,
    pub value: Value,
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Bool(bool),
    Numeric(f64),
}
