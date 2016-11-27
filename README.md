# Jump Consistent Hash

[![Build Status](https://travis-ci.org/jeromefroe/jmphash-rs.svg?branch=master)](https://travis-ci.org/jeromefroe/jmphash-rs)
[![Coverage Status](https://coveralls.io/repos/github/jeromefroe/jmphash-rs/badge.svg?branch=master)](https://coveralls.io/github/jeromefroe/jmphash-rs?branch=master)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/jeromefroe/jmphash-rs/master/LICENSE)

An implementation of the jump consistent hash algorithim as described in
[A Fast, Minimal Memory, Consistent Hash Algorithim] (https://arxiv.org/pdf/1406.2294v1.pdf).

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