# Setting up

I don't know how python packaging works.

First answer is use `pip3 install .` and not `setup.py` or anything, cool.

Then I run into a mac path thing:

https://github.com/DavidBuchanan314/dag-cbrrr/issues/4

Then I run into a weird python thing:

https://github.com/DavidBuchanan314/dag-cbor-benchmark/issues/1

I opened a PR that I think fixes it:

https://github.com/DavidBuchanan314/dag-cbor-benchmark/pull/2

Okay, after a quick PR, I have my first result with JUST `dag-cbrrr`!

## Results (just dag-cbrrr)

```
Hello World Decode:
===================
cbrrr    : 256 ns

Hello World Encode:
===================
cbrrr    : 185 ns

Realistic Decode Tests:
=======================
citm_catalog.json.dagcbor      cbrrr    : 1.88 ms (173.60 MB/s)
canada.json.dagcbor            cbrrr    : 2.70 ms (372.72 MB/s)
twitter.json.dagcbor           cbrrr    : 1.08 ms (354.61 MB/s)

Realistic Encode Tests:
=======================
citm_catalog.json.dagcbor      cbrrr    : 1.08 ms (302.07 MB/s)
canada.json.dagcbor            cbrrr    : 0.81 ms (1241.81 MB/s)
twitter.json.dagcbor           cbrrr    : 0.56 ms (687.03 MB/s)

Decode Torture Tests:
=====================
torture_cids.dagcbor           cbrrr     50.2 ms (77.84 MB/s)
torture_nested_lists.dagcbor   cbrrr     1371.1 ms (6.96 MB/s)
torture_nested_maps.dagcbor    cbrrr     2389.1 ms (7.98 MB/s)
```

## Installing the others

Actually, the other two deps installed with `pip3 install .` with no issues.

## Running the benchmark

This time I get:

```
Hello World Decode:
===================
cbrrr    : 256 ns
libipld  : 494 ns
dag_cbor : 6985 ns

Hello World Encode:
===================
cbrrr    : 157 ns
dag_cbor : 7708 ns

Realistic Decode Tests:
=======================
citm_catalog.json.dagcbor      cbrrr    : 1.72 ms (190.00 MB/s)
citm_catalog.json.dagcbor      libipld  : 17.39 ms (18.77 MB/s)
citm_catalog.json.dagcbor      dag_cbor : 39.99 ms (8.16 MB/s)
canada.json.dagcbor            cbrrr    : 2.53 ms (397.36 MB/s)
canada.json.dagcbor            libipld  : 25.07 ms (40.18 MB/s)
canada.json.dagcbor            dag_cbor : 95.34 ms (10.57 MB/s)
twitter.json.dagcbor           cbrrr    : 1.08 ms (356.95 MB/s)
twitter.json.dagcbor           libipld  : 9.04 ms (42.51 MB/s)
twitter.json.dagcbor           dag_cbor : 17.14 ms (22.41 MB/s)

Realistic Encode Tests:
=======================
citm_catalog.json.dagcbor      cbrrr    : 1.09 ms (299.10 MB/s)
citm_catalog.json.dagcbor      dag_cbor : 54.11 ms (6.03 MB/s)
canada.json.dagcbor            cbrrr    : 0.80 ms (1256.27 MB/s)
canada.json.dagcbor            dag_cbor : 189.65 ms (5.31 MB/s)
twitter.json.dagcbor           cbrrr    : 0.56 ms (681.45 MB/s)
twitter.json.dagcbor           dag_cbor : 21.41 ms (17.94 MB/s)

Decode Torture Tests:
=====================
torture_cids.dagcbor           cbrrr     81.1 ms (48.21 MB/s)
torture_cids.dagcbor           libipld   120.6 ms (32.41 MB/s)
torture_cids.dagcbor           dag_cbor  9450.0 ms (0.41 MB/s)
torture_nested_lists.dagcbor   cbrrr     1450.9 ms (6.57 MB/s)
torture_nested_lists.dagcbor   libipld   SEGFAULT
torture_nested_lists.dagcbor   dag_cbor  ERROR: maximum recursion depth exceeded while calling a Python object
torture_nested_maps.dagcbor    cbrrr     2408.1 ms (7.92 MB/s)
torture_nested_maps.dagcbor    libipld   SEGFAULT
torture_nested_maps.dagcbor    dag_cbor  ERROR: maximum recursion depth exceeded while calling a Python object
```

![segfault image](./segfault-001.png)

Lol, Lmao.
