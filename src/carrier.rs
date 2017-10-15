//! Traits for representing carriers that propagate span contexts across process boundaries.
use std::collections::{HashMap, BTreeMap};
use std::io::{Read, Write};

use Result;
use span::SpanContext;

/// This trait allows to inject `SpanContext` to `TextMap`.
pub trait InjectToTextMap<T>: Sized
where
    T: TextMap,
{
    /// Injects `context` to `carrier`.
    fn inject_to_text_map(context: &SpanContext<Self>, carrier: &mut T) -> Result<()>;
}

/// This trait allows to extract `SpanContext` from `TextMap`.
pub trait ExtractFromTextMap<T>: Sized
where
    T: TextMap,
{
    /// Extracts `SpanContext` from `carrier`.
    ///
    /// If `carrier` contains no span context, it will return `Ok(None)`.
    fn extract_from_text_map(carrier: &T) -> Result<Option<SpanContext<Self>>>;
}

/// This trait represents carriers which support **Text Map** format.
///
/// **Text Map** is an arbitrary string-to-string map with an unrestricted character set
/// for both keys and values.
pub trait TextMap {
    /// Sets the value of `key` in the map to `value`.
    fn set(&mut self, key: &str, value: &str);

    /// Gets the value of `key'.
    fn get(&self, key: &str) -> Option<&str>;
}
impl TextMap for HashMap<String, String> {
    fn set(&mut self, key: &str, value: &str) {
        self.insert(key.to_owned(), value.to_owned());
    }
    fn get(&self, key: &str) -> Option<&str> {
        self.get(key).map(|v| v.as_ref())
    }
}
impl TextMap for BTreeMap<String, String> {
    fn set(&mut self, key: &str, value: &str) {
        self.insert(key.to_owned(), value.to_owned());
    }
    fn get(&self, key: &str) -> Option<&str> {
        self.get(key).map(|v| v.as_ref())
    }
}

/// This trait allows to inject `SpanContext` to HTTP header.
pub trait InjectToHttpHeader<T>: Sized
where
    T: SetHttpHeaderField,
{
    /// Injects `context` to `carrier`.
    fn inject_to_http_header(context: &SpanContext<Self>, carrier: &mut T) -> Result<()>;
}

/// This trait allows to extract `SpanContext` from HTTP header.
pub trait ExtractFromHttpHeader<T>: Sized
where
    T: GetHttpHeaderField,
{
    /// Extracts `SpanContext` from `carrier`.
    ///
    /// If `carrier` contains no span context, it will return `Ok(None)`.
    fn extract_from_http_header(carrier: &T) -> Result<Option<SpanContext<Self>>>;
}

/// This trait allows to insert fields in a HTTP header.
pub trait SetHttpHeaderField {
    /// Sets the value of the field named `name` in the HTTP header to `value`.
    fn set_http_header_field(&mut self, name: &str, value: &str) -> Result<()>;
}

/// This trait allows to get the value of fields in a HTTP header.
pub trait GetHttpHeaderField {
    /// Gets the value of the field named `name` in the HTTP header.
    fn get_http_header_field(&self, name: &str) -> Result<Option<&str>>;
}

/// This trait allows to inject `SpanContext` to binary stream.
pub trait InjectToBinary<T>: Sized
where
    T: Write,
{
    /// Injects `context` to `carrier`.
    fn inject_to_binary(context: &SpanContext<Self>, carrier: &mut T) -> Result<()>;
}

/// This trait allows to extract `SpanContext` from binary stream.
pub trait ExtractFromBinary<T>: Sized
where
    T: Read,
{
    /// Extracts `SpanContext` from `carrier`.
    ///
    /// If `carrier` contains no span context, it will return `Ok(None)`.
    fn extract_from_binary(carrier: &mut T) -> Result<Option<SpanContext<Self>>>;
}
