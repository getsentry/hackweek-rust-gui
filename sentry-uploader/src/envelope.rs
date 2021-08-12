use bytes::Bytes;
use serde_json::Value;

type Map = serde_json::value::Map<String, Value>;

#[derive(Clone, Debug, Default)]
pub struct Envelope {
    buf: Bytes,
    pub header: Map,
    offset: usize,
}

impl Envelope {
    pub fn parse(buf: Vec<u8>) -> Self {
        let buf = Bytes::from(buf);
        let mut stream = serde_json::Deserializer::from_slice(&buf).into_iter();

        let (header, offset) = match stream.next() {
            Some(Ok(h)) => (h, stream.byte_offset()),
            _ => (Default::default(), 0),
        };

        Self {
            buf,
            header,
            offset,
        }
    }

    pub fn into_bytes(self) -> Bytes {
        self.buf
    }

    pub fn items(&self) -> EnvelopeItemIter {
        EnvelopeItemIter {
            buf: self.buf.slice(self.offset..),
        }
    }
}

#[derive(Clone, Debug)]
pub struct EnvelopeItem {
    pub header: Map,
    pub body: Bytes,
}

impl EnvelopeItem {
    pub fn file_name(&self) -> &str {
        self.header
            .get("filename")
            .and_then(|name| name.as_str())
            .or_else(|| match self.header.get("type") {
                Some(Value::String(s)) => match s.as_str() {
                    "event" => Some("Event"),
                    "attachment" => Some("Attachment"),
                    "session" => Some("Session"),
                    "sessions" => Some("Sessions"),
                    "transaction" => Some("Transaction"),
                    _ => None,
                },
                _ => None,
            })
            .unwrap_or("unknown")
    }
}

#[derive(Clone, Debug)]
pub struct EnvelopeItemIter {
    buf: Bytes,
}

impl Iterator for EnvelopeItemIter {
    type Item = EnvelopeItem;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = std::mem::take(&mut self.buf);
        while buf.starts_with(b"\n") {
            buf = buf.slice(1..);
        }

        if buf.is_empty() {
            return None;
        }

        let mut stream = serde_json::Deserializer::from_slice(&buf).into_iter();
        match stream.next() {
            Some(Ok(h)) => {
                let buf = buf.slice(stream.byte_offset() + 1..);
                let len = get_header_len(&h)
                    .or_else(|| buf.iter().position(|b| *b == b'\n'))
                    .unwrap_or_else(|| buf.len());

                let body = buf.slice(..len);
                self.buf = buf.slice(len..);

                Some(EnvelopeItem { header: h, body })
            }
            _ => Some(EnvelopeItem {
                header: Default::default(),
                body: buf,
            }),
        }
    }
}

fn get_header_len(header: &Map) -> Option<usize> {
    Some(header.get("length")?.as_u64()? as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    use sentry_core::protocol::{Attachment, EnvelopeItem, Event};

    fn to_buf(envelope: &sentry_core::Envelope) -> Vec<u8> {
        let mut buf = Vec::new();
        envelope.to_writer(&mut buf).unwrap();
        buf
    }

    #[test]
    fn test_envelope() {
        let mut event = Event::new();
        event.message = Some(String::from("Wow, something bad happened"));
        let mut envelope: sentry_core::Envelope = event.into();
        envelope.add_item(EnvelopeItem::Attachment(Attachment {
            buffer: b"oh hai".to_vec(),
            filename: String::from("main.rs"),
            ty: None,
        }));

        let envelope = Envelope::parse(to_buf(&envelope));

        let mut items_iter = envelope.items();

        let item = items_iter.next().unwrap();
        assert_eq!(
            item.header.get("type").and_then(serde_json::Value::as_str),
            Some("event")
        );
        assert!(item.body.starts_with(br#"{"event_id":""#));

        let item = items_iter.next().unwrap();
        assert_eq!(
            item.header
                .get("filename")
                .and_then(serde_json::Value::as_str),
            Some("main.rs")
        );
        assert_eq!(item.body.as_ref(), b"oh hai");

        assert!(items_iter.next().is_none());
    }
}
