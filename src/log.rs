//! Span log.
#[cfg(feature = "stacktrace")]
use backtrace::Backtrace;
use std::borrow::Cow;
use std::time::SystemTime;

/// Span log builder.
#[derive(Debug)]
pub struct LogBuilder {
    fields: Vec<LogField>,
    time: Option<SystemTime>,
}
impl LogBuilder {
    /// Adds the field.
    pub fn field<T: Into<LogField>>(&mut self, field: T) -> &mut Self {
        self.fields.push(field.into());
        self
    }

    /// Sets the value of timestamp to `time`.
    pub fn time(&mut self, time: SystemTime) -> &mut Self {
        self.time = Some(time);
        self
    }

    /// Returns a specialized builder for the standard log fields.
    pub fn std(&mut self) -> StdLogFieldsBuilder {
        StdLogFieldsBuilder(self)
    }

    /// Returns a specialized builder for the standard error log fields.
    pub fn error(&mut self) -> StdErrorLogFieldsBuilder {
        self.field(LogField::new("event", "error"));
        StdErrorLogFieldsBuilder(self)
    }

    pub(crate) fn new() -> Self {
        LogBuilder {
            fields: Vec::new(),
            time: None,
        }
    }

    pub(crate) fn finish(mut self) -> Option<Log> {
        if self.fields.is_empty() {
            None
        } else {
            self.fields.reverse();
            self.fields.sort_by(|a, b| a.name.cmp(&b.name));
            self.fields.dedup_by(|a, b| a.name == b.name);
            Some(Log {
                fields: self.fields,
                time: self.time.unwrap_or_else(SystemTime::now),
            })
        }
    }
}

/// Span log.
#[derive(Debug, Clone)]
pub struct Log {
    fields: Vec<LogField>,
    time: SystemTime,
}
impl Log {
    /// Returns the fields of this log.
    pub fn fields(&self) -> &[LogField] {
        &self.fields
    }

    /// Returns the timestamp of this log.
    pub fn time(&self) -> SystemTime {
        self.time
    }
}

/// Span log field.
#[derive(Debug, Clone)]
pub struct LogField {
    name: Cow<'static, str>,
    value: Cow<'static, str>,
}
impl LogField {
    /// Makes a new `LogField` instance.
    pub fn new<N, V>(name: N, value: V) -> Self
    where
        N: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        LogField {
            name: name.into(),
            value: value.into(),
        }
    }

    /// Returns the name of this field.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Returns the value of this field.
    pub fn value(&self) -> &str {
        self.value.as_ref()
    }
}
impl<N, V> From<(N, V)> for LogField
where
    N: Into<Cow<'static, str>>,
    V: Into<Cow<'static, str>>,
{
    fn from((n, v): (N, V)) -> Self {
        LogField::new(n, v)
    }
}

/// A specialized span log builder for [the standard log fields].
///
/// [the standard log fields]: https://github.com/opentracing/specification/blob/master/semantic_conventions.md#log-fields-table
#[derive(Debug)]
pub struct StdLogFieldsBuilder<'a>(&'a mut LogBuilder);
impl<'a> StdLogFieldsBuilder<'a> {
    /// Adds the field `LogField::new("event", event)`.
    ///
    /// `event` is a stable identifier for some notable moment in the lifetime of a Span.
    /// For instance, a mutex lock acquisition or release or the sorts of lifetime events
    /// in a browser page load described in the [Performance.timing] specification.
    ///
    /// E.g., from Zipkin, `"cs"`, `"sr"`, `"ss"`, or `"cr"`.
    /// Or, more generally, `"initialized"` or `"timed out"`.
    ///
    /// [Performance.timing]: https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming
    pub fn event<T>(&mut self, event: T) -> &mut Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.0.field(LogField::new("event", event));
        self
    }

    /// Adds the field `LogField::new("message", message)`.
    ///
    /// `message` is a concise, human-readable, one-line message explaining the event.
    ///
    /// E.g., `"Could not connect to backend"`, `"Cache invalidation succeeded"`
    pub fn message<T>(&mut self, message: T) -> &mut Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.0.field(LogField::new("message", message));
        self
    }

    #[cfg(feature = "stacktrace")]
    /// Adds the field `LogField::new("stack", {stack trace})`.
    pub fn stack(&mut self) -> &mut Self {
        self.0
            .field(LogField::new("stack", format!("{:?}", Backtrace::new())));
        self
    }
}

/// A specialized span log builder for [the standard error log fields].
///
/// This builder automatically inserts the field `LogField::new("event", "error")`.
///
/// [the standard error log fields]: https://github.com/opentracing/specification/blob/master/semantic_conventions.md#log-fields-table
#[derive(Debug)]
pub struct StdErrorLogFieldsBuilder<'a>(&'a mut LogBuilder);
impl<'a> StdErrorLogFieldsBuilder<'a> {
    /// Adds the field `LogField::new("error.kind", kind)`.
    ///
    /// `kind` is the type or "kind" of an error.
    ///
    /// E.g., `"Exception"`, `"OSError"`
    pub fn kind<T>(&mut self, kind: T) -> &mut Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.0.field(LogField::new("error.kind", kind));
        self
    }

    /// Adds the field `LogField::new("message", message)`.
    ///
    /// `message` is a concise, human-readable, one-line message explaining the event.
    ///
    /// E.g., `"Could not connect to backend"`, `"Cache invalidation succeeded"`
    pub fn message<T>(&mut self, message: T) -> &mut Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.0.field(LogField::new("message", message));
        self
    }

    #[cfg(feature = "stacktrace")]
    /// Adds the field `LogField::new("stack", {stack trace})`.
    pub fn stack(&mut self) -> &mut Self {
        self.0
            .field(LogField::new("stack", format!("{:?}", Backtrace::new())));
        self
    }
}
