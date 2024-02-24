use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use native_bench::{PyBytes_FromStringAndSize, cbrrr_parse_object};
// use libipld::


const BENCHES: &[(&str, &[u8])] = &[
    ("canada.json.dagcbor", include_bytes!("../../../vendor/dag-cbor-benchmark/data/canada.json.dagcbor")),
    ("citm_catalog.json.dagcbor", include_bytes!("../../../vendor/dag-cbor-benchmark/data/citm_catalog.json.dagcbor")),
    ("torture_cids.dagcbor", include_bytes!("../../../vendor/dag-cbor-benchmark/data/torture_cids.dagcbor")),

    // Benchmarking torture_nested_lists.dagcbor: Warming up for 3.0000 s
    // thread 'main' has overflowed its stack
    // fatal runtime error: stack overflow
    // ("torture_nested_lists.dagcbor", include_bytes!("../../../vendor/dag-cbor-benchmark/data/torture_nested_lists.dagcbor")),

    // Benchmarking torture_nested_maps.dagcbor: Warming up for 3.0000 s
    // thread 'main' has overflowed its stack
    // fatal runtime error: stack overflow
    // ("torture_nested_maps.dagcbor", include_bytes!("../../../vendor/dag-cbor-benchmark/data/torture_nested_maps.dagcbor")),

    ("trivial_helloworld.dagcbor", include_bytes!("../../../vendor/dag-cbor-benchmark/data/trivial_helloworld.dagcbor")),
    ("twitter.json.dagcbor", include_bytes!("../../../vendor/dag-cbor-benchmark/data/twitter.json.dagcbor")),
];

pub fn ipld_bench(c: &mut Criterion) {
    // Compiles, but segfaults, because cid_ctor needs to be more real.
    //
    // unsafe {
    //     let mut outptr: *mut () = core::ptr::null_mut();
    //     let res = cbrrr_parse_object(HELLO_WORLD.as_ptr(), HELLO_WORLD.len(), &mut outptr, core::ptr::null_mut(), 0);
    //     assert!(res >= 0);
    //     assert!(!outptr.is_null());
    // }

    for (fname, fcont) in BENCHES {
        c.bench_function(fname, |b| {
            b.iter(|| ipld_helpers::decode_dag_cbor(black_box(fcont)).unwrap())
        });
    }
}

criterion_group!(benches, ipld_bench);
criterion_main!(benches);

mod brrr_helpers {

}


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
