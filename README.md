# Jump Consistent Hash

[![Build Status](https://travis-ci.org/jeromefroe/jmphash-rs.svg?branch=master)](https://travis-ci.org/jeromefroe/jmphash-rs)
[![codecov](https://codecov.io/gh/jeromefroe/jmphash-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/jeromefroe/jmphash-rs)
[![crates.io](https://img.shields.io/crates/v/jmphash.svg)](https://crates.io/crates/jmphash/)
[![docs.rs](https://docs.rs/jmphash/badge.svg)](https://docs.rs/jmphash/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/jeromefroe/jmphash-rs/master/LICENSE)

[Documentation](https://docs.rs/jmphash/)

An implementation of the jump consistent hash algorithim as described in
[A Fast, Minimal Memory, Consistent Hash Algorithim](https://arxiv.org/pdf/1406.2294v1.pdf).

## Example

``` rust
extern crate jmphash;

use jmphash::jump_hash;

fn really_complicated_hash_function() -> u64 {
  42
}

fn main() {
    let num_buckets = 100;

    // first use a hash function of your choice to create a u64 key
    let key = really_complicated_hash_function();

    // then one can use `jump_hash` to map the key to a bucket
    let bucket = jump_hash(key, num_buckets);
}
```