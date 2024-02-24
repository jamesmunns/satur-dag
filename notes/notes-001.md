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

## Running in Rust

```
   Compiling native-bench v0.1.0 (/Users/james/personal/satur-dag/source/native-bench)
    Finished bench [optimized] target(s) in 1.00s
     Running unittests src/lib.rs (target/release/deps/native_bench-67be11b117d8ed6e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/smoke.rs (target/release/deps/smoke-e4b82648cc87279f)
Gnuplot not found, using plotters backend
canada.json.dagcbor     time:   [5.9365 ms 5.9542 ms 5.9745 ms]
                        change: [-2.1747% -1.3811% -0.6268%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe

citm_catalog.json.dagcbor
                        time:   [6.4516 ms 6.4690 ms 6.4912 ms]
                        change: [-5.2973% -3.6812% -2.0849%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  3 (3.00%) high severe

torture_cids.dagcbor    time:   [7.1196 µs 7.1318 µs 7.1436 µs]
                        change: [-0.6665% -0.3871% -0.1100%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) low mild
  4 (4.00%) high mild

trivial_helloworld.dagcbor
                        time:   [73.030 ns 73.086 ns 73.144 ns]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe

twitter.json.dagcbor    time:   [2.2873 ms 2.2900 ms 2.2933 ms]
Found 14 outliers among 100 measurements (14.00%)
  8 (8.00%) high mild
  6 (6.00%) high severe
```

## Oranges to Apples

These are not reasonable comparisons. There's a LOT going on in python, particularly creating a ton of
allocations.

| file                  | ipld in python    | ipld native   | cbrrr         |
| :---                  | :---              | :---          | :---          |
| trivial_helloworld    | 494 ns            | 73.086 ns     | 256 ns        |
| citm_catalog          | 17.39 ms          | 6.4690 ms     | 1.72 ms       |
| canada                | 25.07 ms          | 5.9542 ms     | 2.53 ms       |
| twitter               | 9.04 ms           | 2.2900 ms     | 1.08 ms       |
| torture_cids          | 120.6 ms          | 7.1318 µs     | 81.1 ms       |
| torture_nested_lists  | DNF               | DNF           | 1450.9 ms     |
| torture_nested_maps   | DNF               | DNF           | 2408.1 ms     |

## DNF

```
Benchmarking torture_nested_lists.dagcbor: Warming up for 3.0000 s
thread 'main' has overflowed its stack
fatal runtime error: stack overflow

Benchmarking torture_nested_maps.dagcbor: Warming up for 3.0000 s
thread 'main' has overflowed its stack
fatal runtime error: stack overflow
```
