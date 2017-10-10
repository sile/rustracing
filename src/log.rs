use std::borrow::Cow;
use std::error::Error;
use std::time::SystemTime;
use backtrace::Backtrace;

#[derive(Debug)]
pub struct LogBuilder {
    fields: Vec<LogField>,
    time: Option<SystemTime>,
}
impl LogBuilder {
    pub(crate) fn new() -> Self {
        LogBuilder {
            fields: Vec::new(),
            time: None,
        }
    }
    pub fn field<T: Into<LogField>>(&mut self, field: T) -> &mut Self {
        self.fields.push(field.into());
        self
    }
    pub fn time(&mut self, time: SystemTime) -> &mut Self {
        self.time = Some(time);
        self
    }
    pub fn std(&mut self) -> StdLogFieldsBuilder {
        StdLogFieldsBuilder(self)
    }
    pub fn error(&mut self) -> StdErrorLogFieldsBuilder {
        self.field(LogField::new("event", "error"));
        StdErrorLogFieldsBuilder(self)
    }
    pub(crate) fn finish(mut self) -> Option<Log> {
        if self.fields.is_empty() {
            None
        } else {
            self.fields.reverse();
            self.fields.sort_by(|a, b| a.key.cmp(&b.key));
            self.fields.dedup_by(|a, b| a.key == b.key);
            Some(Log {
                fields: self.fields,
                time: self.time.unwrap_or_else(|| SystemTime::now()),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct Log {
    fields: Vec<LogField>,
    time: SystemTime,
}
impl Log {
    pub fn fields(&self) -> &[LogField] {
        &self.fields
    }
    pub fn time(&self) -> SystemTime {
        self.time
    }
}

#[derive(Debug, Clone)]
pub struct LogField {
    key: Cow<'static, str>,
    value: Cow<'static, str>,
}
impl LogField {
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        LogField {
            key: key.into(),
            value: value.into(),
        }
    }
    pub fn key(&self) -> &str {
        self.key.as_ref()
    }
    pub fn value(&self) -> &str {
        self.value.as_ref()
    }
}
impl<K, V> From<(K, V)> for LogField
where
    K: Into<Cow<'static, str>>,
    V: Into<Cow<'static, str>>,
{
    fn from((k, v): (K, V)) -> Self {
        LogField::new(k, v)
    }
}

#[derive(Debug)]
pub struct StdLogFieldsBuilder<'a>(&'a mut LogBuilder);
impl<'a> StdLogFieldsBuilder<'a> {
    pub fn event<T>(&mut self, event: T) -> &mut Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.0.field(LogField::new("event", event));
        self
    }
    pub fn message<T>(&mut self, message: T) -> &mut Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.0.field(LogField::new("message", message));
        self
    }
    pub fn stack(&mut self) -> &mut Self {
        self.0.field(LogField::new(
            "stack",
            format!("{:?}", Backtrace::new()),
        ));
        self
    }
}

#[derive(Debug)]
pub struct StdErrorLogFieldsBuilder<'a>(&'a mut LogBuilder);
impl<'a> StdErrorLogFieldsBuilder<'a> {
    pub fn kind<T>(&mut self, kind: T) -> &mut Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.0.field(LogField::new("error.kind", kind));
        self
    }
    pub fn object<T: Error>(&mut self, error: T) -> &mut Self {
        self.0.field(
            LogField::new("error.object", error.to_string()),
        );
        self
    }
    pub fn message<T>(&mut self, message: T) -> &mut Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.0.field(LogField::new("message", message));
        self
    }
    pub fn stack(&mut self) -> &mut Self {
        self.0.field(LogField::new(
            "stack",
            format!("{:?}", Backtrace::new()),
        ));
        self
    }
}

#[derive(Debug)]
pub struct StdLogFieldsField;
impl StdLogFieldsField {
    pub fn error_kind<V>(value: V) -> LogField
    where
        V: Into<Cow<'static, str>>,
    {
        LogField::new("error.kind", value)
    }
    pub fn error_object<V: Error>(value: V) -> LogField {
        LogField::new("error.object", value.to_string())
    }
    pub fn event<V>(value: V) -> LogField
    where
        V: Into<Cow<'static, str>>,
    {
        LogField::new("event", value)
    }
    pub fn message<V>(value: V) -> LogField
    where
        V: Into<Cow<'static, str>>,
    {
        LogField::new("message", value)
    }
    pub fn statck<V>(value: V) -> LogField
    where
        V: Into<Cow<'static, str>>,
    {
        LogField::new("statc,", value)
    }
}
