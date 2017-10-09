use std::borrow::Cow;
use std::ops::Deref;
use std::sync::mpsc;
use std::time::SystemTime;

#[derive(Debug)]
pub struct Span<T>(Option<SpanInner<T>>);
impl<T> Span<T> {
    pub fn disabled() -> Self {
        Span(None)
    }
    pub fn is_enabled(&self) -> bool {
        self.0.is_some()
    }
    pub fn set_operation_name(&mut self, name: Cow<'static, str>) {
        if let Some(inner) = self.0.as_mut() {
            inner.operation_name = name.into();
        }
    }
    pub fn set_finish_time(&mut self, time: SystemTime) {
        if let Some(inner) = self.0.as_mut() {
            inner.finish_time = Some(time);
        }
    }
    pub fn set_tag(&mut self, tag: SpanTag) {
        if let Some(inner) = self.0.as_mut() {
            inner.tags.retain(|x| x.key != tag.key);
            inner.tags.push(tag);
        }
    }
    pub fn set_baggage_item(&mut self, item: BaggageItem) {
        if let Some(inner) = self.0.as_mut() {
            inner.context.baggage_items.retain(|x| x.key != item.key);
            inner.context.baggage_items.push(item);
        }
    }
    pub fn get_baggage_item(&self, key: &str) -> Option<&BaggageItem> {
        if let Some(inner) = self.0.as_ref() {
            inner.context.baggage_items.iter().find(|x| x.key == key)
        } else {
            None
        }
    }
    pub fn log(&mut self, record: SpanLogRecord) {
        if let Some(inner) = self.0.as_mut() {
            inner.logs.push(record);
        }
    }
}
impl<T> Drop for Span<T> {
    fn drop(&mut self) {
        if let Some(inner) = self.0.take() {
            let finished = FinishedSpan {
                operation_name: inner.operation_name,
                start_time: inner.start_time,
                finish_time: inner.finish_time.unwrap_or_else(|| SystemTime::now()),
                references: inner.references,
                tags: inner.tags,
                logs: inner.logs,
                context: inner.context,
            };
            let _ = inner.span_tx.send(finished);
        }
    }
}

#[derive(Debug)]
struct SpanInner<T> {
    operation_name: Cow<'static, str>,
    start_time: SystemTime,
    finish_time: Option<SystemTime>,
    references: Vec<SpanReference<T>>,
    tags: Vec<SpanTag>,
    logs: Vec<SpanLogRecord>,
    context: SpanContext<T>,
    span_tx: mpsc::Sender<FinishedSpan<T>>,
}

#[derive(Debug)]
pub struct FinishedSpan<T> {
    operation_name: Cow<'static, str>,
    start_time: SystemTime,
    finish_time: SystemTime,
    references: Vec<SpanReference<T>>,
    tags: Vec<SpanTag>,
    logs: Vec<SpanLogRecord>,
    context: SpanContext<T>,
}
impl<T> FinishedSpan<T> {
    pub fn operation_name(&self) -> &str {
        self.operation_name.as_ref()
    }
    pub fn start_time(&self) -> SystemTime {
        self.start_time
    }
    pub fn finish_time(&self) -> SystemTime {
        self.finish_time
    }
    pub fn logs(&self) -> &[SpanLogRecord] {
        &self.logs
    }
    pub fn tags(&self) -> &[SpanTag] {
        &self.tags
    }
    pub fn references(&self) -> &[SpanReference<T>] {
        &self.references
    }
    pub fn context(&self) -> &SpanContext<T> {
        &self.context
    }
}

#[derive(Debug)]
pub struct SpanTag {
    pub key: Cow<'static, str>,
    pub value: SpanTagValue,
}

#[derive(Debug)]
pub enum SpanTagValue {
    String(Cow<'static, str>),
    Boolean(bool),
    Integer(i64),
    Float(f64),
}

#[derive(Debug)]
pub struct SpanLogRecord {
    time: SystemTime,
    fields: Vec<SpanLogField>,
}
impl SpanLogRecord {
    pub fn time(&self) -> SystemTime {
        self.time
    }
    pub fn fields(&self) -> &[SpanLogField] {
        &self.fields
    }
}

#[derive(Debug)]
pub struct SpanLogField {
    pub key: Cow<'static, str>,
    pub value: Cow<'static, str>,
}

#[derive(Debug, Clone)]
pub struct SpanContext<T> {
    state: T,
    baggage_items: Vec<BaggageItem>,
}
impl<T> SpanContext<T> {
    pub fn new<I>(state: T, mut baggage_items: Vec<BaggageItem>) -> Self
    where
        I: Iterator<Item = BaggageItem>,
    {
        baggage_items.sort_by(|a, b| a.key.cmp(&b.key));
        baggage_items.dedup_by(|a, b| a.key == b.key);
        SpanContext {
            state,
            baggage_items,
        }
    }
    pub fn state(&self) -> &T {
        &self.state
    }
    pub fn baggage_items(&self) -> &[BaggageItem] {
        &self.baggage_items
    }
}

#[derive(Debug, Clone)]
pub struct BaggageItem {
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub enum SpanReference<T> {
    ChildOf(SpanContext<T>),
    FollowsFrom(SpanContext<T>),
}
impl<T> Deref for SpanReference<T> {
    type Target = SpanContext<T>;
    fn deref(&self) -> &Self::Target {
        match *self {
            SpanReference::ChildOf(ref s) => s,
            SpanReference::FollowsFrom(ref s) => s,
        }
    }
}
