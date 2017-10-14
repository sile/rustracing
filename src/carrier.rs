use std::collections::{HashMap, BTreeMap};
use std::io::{Read, Write};

use Result;
use span::SpanContext;

pub trait InjectToTextMap<T>: Sized
where
    T: TextMap,
{
    fn inject_to_text_map(this: &SpanContext<Self>, carrier: &mut T) -> Result<()>;
}

pub trait ExtractFromTextMap<T>: Sized
where
    T: TextMap,
{
    fn extract_from_text_map(carrier: &T) -> Result<Option<SpanContext<Self>>>;
}

pub trait TextMap {
    fn set(&mut self, key: &str, value: &str);
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

pub trait InjectToHttpHeader<T>: Sized
where
    T: SetHttpHeaderField,
{
    fn inject_to_http_header(this: &SpanContext<Self>, carrier: &mut T) -> Result<()>;
}

pub trait ExtractFromHttpHeader<T>: Sized
where
    T: GetHttpHeaderField,
{
    fn extract_from_http_header(carrier: &T) -> Result<Option<SpanContext<Self>>>;
}

pub trait SetHttpHeaderField {
    fn set_http_header_field(&mut self, key: &str, value: &str) -> Result<()>;
}

pub trait GetHttpHeaderField {
    fn get_http_header_field(&self, key: &str) -> Result<Option<&str>>;
}

pub trait InjectToBinary<T>: Sized
where
    T: Write,
{
    fn inject_to_binary(this: &SpanContext<Self>, carrier: &mut T) -> Result<()>;
}

pub trait ExtractFromBinary<T>: Sized
where
    T: Read,
{
    fn extract_from_binary(carrier: &mut T) -> Result<Option<SpanContext<Self>>>;
}
