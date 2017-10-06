use std::time::SystemTime;

use tag::Tag;

#[derive(Debug)]
pub struct Span {
    pub operation_name: String,
    pub start_time: SystemTime,
    pub finish_time: SystemTime,
    pub tags: Vec<Tag>,
    pub logs: Vec<SpanLog>,
    pub context: SpanContext,
    pub references: Vec<Reference>,
}
impl Span {
    /// Returns the `SpanContext` for the given `Span`.
    ///
    /// The returned value may be used even after the `Span` is finished.
    pub fn context(&self) -> &SpanContext {
        &self.context
    }

    pub fn set_operation_name(&mut self, name: &str) {
        self.operation_name = name.to_owned();
    }

    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.push(tag);
    }
    pub fn add_baggage_item(&mut self, _item: BaggageItem) {}
    pub fn get_baggage_item(&self, key: &str) -> Option<&BaggageItem> {
        unimplemented!("key:{}", key);
    }
    pub fn log(&mut self, _log: SpanLog, _time: Option<SystemTime>) {}
    pub fn finish(self, _time: Option<SystemTime>) {}
}

// See: https://github.com/opentracing/specification/blob/master/semantic_conventions.md
// (standard log keys)
#[derive(Debug)]
pub struct SpanLog {
    pub key: String,
    pub value: Vec<u8>, // TODO
}

#[derive(Debug)]
pub struct SpanContext {
    pub impl_dependent_state: Vec<u8>, // TODO
    pub baggage_items: Vec<BaggageItem>,
}
impl SpanContext {
    pub fn baggage_items(&self) -> &[BaggageItem] {
        &self.baggage_items
    }
}

// Baggage items are key:value string pairs that apply to the given `Span`,
// its `SpanContext`, and all `Spans` which directly or transitively reference the local `Span`.
// That is, baggage items propagate in-band along with the trace itself.
#[derive(Debug)]
pub struct BaggageItem {
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub enum Reference {
    ChildOf(SpanContext),
    FollowsFrom(SpanContext),
}
