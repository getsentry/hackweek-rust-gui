use serde_json::Value;

type Map = serde_json::value::Map<String, Value>;

#[derive(Clone, Debug, Default)]
pub struct Envelope {
    buf: Vec<u8>,
    pub header: Map,
    offset: usize,
}

impl Envelope {
    pub fn parse(buf: Vec<u8>) -> Self {
        let mut stream = serde_json::Deserializer::from_slice(&buf).into_iter();

        let (header, offset) = match stream.next() {
            Some(Ok(h)) => (h, stream.byte_offset() + 1),
            _ => (Default::default(), 0),
        };

        Self {
            buf,
            header,
            offset,
        }
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.buf
    }

    pub fn items(&self) -> EnvelopeItemIter {
        EnvelopeItemIter {
            buf: &self.buf[self.offset..],
        }
    }
}

#[derive(Clone, Debug)]
pub struct EnvelopeItem<'buf> {
    pub header: Map,
    pub body: &'buf [u8],
}

impl EnvelopeItem<'_> {
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
                    _ => unreachable!(),
                },
                _ => None,
            })
            .unwrap_or("unknown")
    }
}

#[derive(Copy, Clone, Debug)]
pub struct EnvelopeItemIter<'buf> {
    buf: &'buf [u8],
}

impl<'buf> Iterator for EnvelopeItemIter<'buf> {
    type Item = EnvelopeItem<'buf>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = self.buf;
        while buf.starts_with(b"\n") {
            buf = &buf[1..];
        }

        if buf.is_empty() {
            return None;
        }

        let mut stream = serde_json::Deserializer::from_slice(buf).into_iter();
        match stream.next() {
            Some(Ok(h)) => {
                let buf = &buf[stream.byte_offset() + 1..];
                let len = get_header_len(&h)
                    .or_else(|| buf.iter().position(|b| *b == b'\n'))
                    .unwrap_or(buf.len());

                let (body, rest) = buf.split_at(len);
                self.buf = rest;

                Some(EnvelopeItem { header: h, body })
            }
            _ => {
                let item = Some(EnvelopeItem {
                    header: Default::default(),
                    body: self.buf,
                });
                self.buf = &[];
                item
            }
        }
    }
}

fn get_header_len(header: &Map) -> Option<usize> {
    Some(header.get("length")?.as_u64()? as usize)
}
