use std::borrow::Cow;
use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct Tag {
    key: Cow<'static, str>,
    value: TagValue,
}
impl Tag {
    /// # Examples
    ///
    /// ```
    /// use rustracing::tag::{Tag, TagValue};
    ///
    /// let tag = Tag::new("foo", "bar");
    /// assert_eq!(tag.key(), "foo");
    /// assert_eq!(tag.value(), &TagValue::from("bar"));
    /// ```
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: Into<TagValue>,
    {
        Tag {
            key: key.into(),
            value: value.into(),
        }
    }

    pub fn key(&self) -> &str {
        self.key.as_ref()
    }
    pub fn value(&self) -> &TagValue {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

#[derive(Debug)]
pub struct StdTag;
impl StdTag {
    pub fn component<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("component", value.into())
    }
    pub fn db_instance<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("db.instance", value.into())
    }
    pub fn db_statement<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("db.statement", value.into())
    }
    pub fn db_type<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("db.type", value.into())
    }
    pub fn db_user<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("db.user", value.into())
    }
    pub fn error() -> Tag {
        Tag::new("error", true)
    }
    pub fn http_method<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("http.method", value.into())
    }
    pub fn http_status_code(value: u16) -> Tag {
        Tag::new("http.status_code", value as i64)
    }
    pub fn http_url<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("http.url", value.into())
    }
    pub fn message_bus_destination<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("message_bus.destination", value.into())
    }
    pub fn peer_address<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("peer.address", value.into())
    }
    pub fn peer_hostname<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("peer.hostname", value.into())
    }
    pub fn peer_ip(value: IpAddr) -> Tag {
        match value {
            IpAddr::V4(v) => Tag::new("peer.ipv4", v.to_string()),
            IpAddr::V6(v) => Tag::new("peer.ipv6", v.to_string()),
        }
    }
    pub fn peer_port(value: u16) -> Tag {
        Tag::new("peer.port", value as i64)
    }
    pub fn peer_service<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("peer.service", value.into())
    }
    pub fn sampling_priority(value: u32) -> Tag {
        Tag::new("sampling.priority", value as i64)
    }
    pub fn span_kind<V>(value: V) -> Tag
    where
        V: Into<Cow<'static, str>>,
    {
        Tag::new("span.kind", value.into())
    }
}
