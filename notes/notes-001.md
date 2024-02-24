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
