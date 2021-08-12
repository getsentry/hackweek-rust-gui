type Map = serde_json::value::Map<String, serde_json::Value>;

pub struct Envelope<'buf> {
    pub header: Map,
    body: &'buf [u8],
}

impl<'buf> Envelope<'buf> {
    pub fn parse(mut buf: &'buf [u8]) -> Self {
        let mut stream = serde_json::Deserializer::from_slice(buf).into_iter();

        let header = match stream.next() {
            Some(Ok(h)) => {
                buf = &buf[stream.byte_offset()..];
                h
            }
            _ => Default::default(),
        };

        Self { header, body: buf }
    }
}

impl<'buf> Envelope<'buf> {
    pub fn items(&self) -> EnvelopeItemIter<'buf> {
        EnvelopeItemIter { buf: self.body }
    }
}

pub struct EnvelopeItem<'buf> {
    pub header: Map,
    pub body: &'buf [u8],
}

pub struct EnvelopeItemIter<'buf> {
    buf: &'buf [u8],
}

impl<'buf> Iterator for EnvelopeItemIter<'buf> {
    type Item = EnvelopeItem<'buf>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.is_empty() {
            return None;
        }

        let mut stream = serde_json::Deserializer::from_slice(self.buf).into_iter();
        match stream.next() {
            Some(Ok(h)) => {
                let buf = &self.buf[stream.byte_offset()..];
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
