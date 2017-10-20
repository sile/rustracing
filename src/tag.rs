//! Span tag.
use std::borrow::Cow;
use std::net::{IpAddr, SocketAddr};

/// Span tag.
#[derive(Debug, Clone)]
pub struct Tag {
    name: Cow<'static, str>,
    value: TagValue,
}
impl Tag {
    /// # Examples
    ///
    /// ```
    /// use rustracing::tag::{Tag, TagValue};
    ///
    /// let tag = Tag::new("foo", "bar");
    /// assert_eq!(tag.name(), "foo");
    /// assert_eq!(tag.value(), &TagValue::from("bar"));
    /// ```
    pub fn new<N, V>(name: N, value: V) -> Self
    where
        N: Into<Cow<'static, str>>,
        V: Into<TagValue>,
    {
        Tag {
            name: name.into(),
            value: value.into(),
        }
    }

    /// Returns the name of this tag.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Returns the value of this tag.
    pub fn value(&self) -> &TagValue {
        &self.value
    }
}

/// Span tag value.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[allow(missing_docs)]
pub enum TagValue {
    String(Cow<'static, str>),
    Boolean(bool),
    Integer(i64),
    Float(f64),
}
impl From<&'static str> for TagValue {
    fn from(f: &'static str) -> Self {
        TagValue::String(Cow::Borrowed(f))
    }
}
impl From<String> for TagValue {
    fn from(f: String) -> Self {
        TagValue::String(Cow::Owned(f))
    }
}
impl From<Cow<'static, str>> for TagValue {
    fn from(f: Cow<'static, str>) -> Self {
        TagValue::String(f)
    }
}
impl From<bool> for TagValue {
    fn from(f: bool) -> Self {
        TagValue::Boolean(f)
    }
}
impl From<i64> for TagValue {
    fn from(f: i64) -> Self {
        TagValue::Integer(f)
    }
}
impl From<f64> for TagValue {
    fn from(f: f64) -> Self {
        TagValue::Float(f)
    }
}

/// [Standard span tags][tags].
/// [tags]: https://github.com/opentracing/specification/blob/master/semantic_conventions.md#span-tags-table
#[derive(Debug)]
pub struct StdTag;
impl StdTag {
    /// Makes a `"component"` tag.
    ///
    /// It indicates the software package, framework, library,
    /// or module that generated the associated `Span`.
    ///
    /// E.g., `"grpc"`, `"django"`, `"JDBI"`.
    pub fn component<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("component", value.into())
    }

    /// Makes a `"db.instance"` tag.
    ///
    /// It indicates database instance name.
    ///
    /// E.g., In java, if the jdbc.url=`"jdbc:mysql://127.0.0.1:3306/customers"`,
    /// the instance name is `"customers"`.
    pub fn db_instance<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("db.instance", value.into())
    }

    /// Makes a `"db.statement"` tag.
    ///
    /// It indicates a database statement for the given database type.
    ///
    /// E.g.,
    /// for db.type=`"sql"`, `"SELECT * FROM wuser_table"`;
    /// for db.type=`"redis"`, `"SET mykey 'WuValue'"`.
    pub fn db_statement<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("db.statement", value.into())
    }

    /// Makes a `"db.type"` tag.
    ///
    /// It indicates database type.
    ///
    /// For any SQL database, `"sql"`.
    /// For others, the lower-case database category, e.g. `"cassandra"`, `"hbase"`, or `"redis"`.
    pub fn db_type<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("db.type", value.into())
    }

    /// Makes a `"db.user"` tag.
    ///
    /// It indicates username for accessing database.
    ///
    /// E.g., `"readonly_user"` or `"reporting_user"`
    pub fn db_user<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("db.user", value.into())
    }

    /// Makes a `"error"` tag that has the value `true`.
    ///
    /// It indicates the application considers the operation represented by the `Span` to have failed.
    pub fn error() -> Tag {
        Tag::new("error", true)
    }

    /// Makes a `"http.method"` tag.
    ///
    /// It indicates HTTP method of the request for the associated `Span`.
    ///
    /// E.g., `"GET"`, `"POST"`
    pub fn http_method<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("http.method", value.into())
    }

    /// Makes a `"http.status_code"` tag.
    ///
    /// It indicates HTTP response status code for the associated `Span`.
    ///
    /// E.g., 200, 503, 404
    pub fn http_status_code(value: u16) -> Tag {
        Tag::new("http.status_code", i64::from(value))
    }

    /// Makes a `"http.url"` tag.
    ///
    /// It indicates URL of the request being handled in this segment of the trace, in standard URI format.
    ///
    /// E.g., `"https://domain.net/path/to?resource=here"`
    pub fn http_url<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("http.url", value.into())
    }

    /// Makes a `"message_bus.destination" tag.
    ///
    /// It indicates an address at which messages can be exchanged.
    ///
    /// E.g. A Kafka record has an associated `"topic name"` that can be extracted by
    /// the instrumented producer or consumer and stored using this tag.
    pub fn message_bus_destination<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("message_bus.destination", value.into())
    }

    /// Makes a `"peer.address"` tag.
    ///
    /// It indicates remote "address", suitable for use in a networking client library.
    ///
    /// This may be a `"ip:port"`, a bare `"hostname"`, a FQDN,
    /// or even a JDBC substring like `"mysql://prod-db:3306"`.
    pub fn peer_address<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("peer.address", value.into())
    }

    /// Makes a `"peer.hostname"` tag.
    ///
    /// It indicates remote hostname.
    ///
    /// E.g., `"opentracing.io"`, `"internal.dns.name"`
    pub fn peer_hostname<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("peer.hostname", value.into())
    }

    /// Makes a `"peer.ipXX"` and `"peer.port"` tags.
    pub fn peer_addr(value: SocketAddr) -> Vec<Tag> {
        vec![Self::peer_ip(value.ip()), Self::peer_port(value.port())]
    }

    /// Makes a tag which has the name either `"peer.ipv4"` or `"peer.ipv6"` depending on the value.
    ///
    /// It indicates remote IP address.
    ///
    /// E.g., `"127.0.0.1"`, `"2001:0db8:85a3:0000:0000:8a2e:0370:7334"`
    pub fn peer_ip(value: IpAddr) -> Tag {
        match value {
            IpAddr::V4(v) => Tag::new("peer.ipv4", v.to_string()),
            IpAddr::V6(v) => Tag::new("peer.ipv6", v.to_string()),
        }
    }

    /// Makes a `"peer.port"` tag.
    ///
    /// It indicates remote port.
    ///
    /// E.g., `80`
    pub fn peer_port(value: u16) -> Tag {
        Tag::new("peer.port", i64::from(value))
    }

    /// Makes a `"peer.service"` tag.
    ///
    /// It indicates remote service name (for some unspecified definition of `"service"`).
    ///
    /// E.g., `"elasticsearch"`, `"a_custom_microservice"`, `"memcache"`
    pub fn peer_service<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("peer.service", value.into())
    }

    /// Makes a `"samplingpriority"` tag.
    ///
    /// If greater than `0`, a hint to the `Tracer` to do its best to capture the trace.
    /// If `0`, a hint to the trace to not-capture the trace.
    /// If absent, the `Tracer` should use its default sampling mechanism.
    pub fn sampling_priority(value: u32) -> Tag {
        Tag::new("sampling.priority", i64::from(value))
    }

    /// Makes a `"span.ind"` tag.
    ///
    /// Either `"client"` or `"server"` for the appropriate roles in an RPC,
    /// and `"producer"` or `"consumer"` for the appropriate roles in a messaging scenario.
    pub fn span_kind<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("span.kind", value.into())
    }
}
