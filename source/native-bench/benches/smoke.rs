use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use libipld::

const HELLO_WORLD: &[u8] =
    include_bytes!("../../../vendor/dag-cbor-benchmark/data/trivial_helloworld.dagcbor");

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("ipld hello world", |b| {
        b.iter(|| ipld_helpers::decode_dag_cbor(black_box(HELLO_WORLD)).unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

mod ipld_helpers {
    use std::{
        borrow::Cow,
        collections::HashMap,
        io::{BufReader, Cursor, Read, Seek},
    };

    use libipld::{
        cbor::{cbor::MajorKind, decode},
        Ipld,
    };

    use anyhow::Result;

    pub fn decode_dag_cbor(data: &[u8]) -> Result<HashMapItem> {
        let mut reader = BufReader::new(Cursor::new(data));
        let data = parse_dag_cbor_object(&mut reader)?;
        Ok(ipld_to_hashmap(data))
    }

    #[derive(Clone, PartialEq)]
    pub enum HashMapItem {
        Null,
        Bool(bool),
        Integer(i128),
        Float(f64),
        String(String),
        List(Vec<HashMapItem>),
        Map(HashMap<String, HashMapItem>),
        Bytes(Cow<'static, [u8]>),
    }

    fn ipld_to_hashmap(x: Ipld) -> HashMapItem {
        match x {
            Ipld::Null => HashMapItem::Null,
            Ipld::Bool(b) => HashMapItem::Bool(b),
            Ipld::Integer(i) => HashMapItem::Integer(i),
            Ipld::Float(f) => HashMapItem::Float(f),
            Ipld::String(s) => HashMapItem::String(s),
            Ipld::Bytes(b) => HashMapItem::Bytes(Cow::Owned(b)),
            Ipld::List(l) => HashMapItem::List(l.into_iter().map(ipld_to_hashmap).collect()),
            Ipld::Map(m) => HashMapItem::Map(
                m.into_iter()
                    .map(|(k, v)| (k, ipld_to_hashmap(v)))
                    .collect(),
            ),
            Ipld::Link(cid) => HashMapItem::String(cid.to_string()),
        }
    }

    fn parse_dag_cbor_object<R: Read + Seek>(mut reader: &mut BufReader<R>) -> Result<Ipld> {
        let major = decode::read_major(&mut reader)?;
        Ok(match major.kind() {
            MajorKind::UnsignedInt | MajorKind::NegativeInt => Ipld::Integer(major.info() as i128),
            MajorKind::ByteString => {
                Ipld::Bytes(decode::read_bytes(&mut reader, major.info() as u64)?)
            }
            MajorKind::TextString => {
                Ipld::String(decode::read_str(&mut reader, major.info() as u64)?)
            }
            MajorKind::Array => Ipld::List(decode::read_list(&mut reader, major.info() as u64)?),
            MajorKind::Map => Ipld::Map(decode::read_map(&mut reader, major.info() as u64)?),
            MajorKind::Tag => {
                if major.info() != 42 {
                    return Err(anyhow::anyhow!("non-42 tags are not supported"));
                }

                parse_dag_cbor_object(reader)?
            }
            MajorKind::Other => Ipld::Null,
        })
    }
}
