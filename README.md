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