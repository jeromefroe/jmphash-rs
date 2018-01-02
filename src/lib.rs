// MIT License

// Copyright (c) 2016 Jerome Froelich

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! An implementation of the jump consistent hash algorithim as described in
//! [A Fast, Minimal Memory, Consistent Hash Algorithm](https://arxiv.org/pdf/1406.2294v1.pdf).
//!
//! ## Example
//!
//! ``` rust,no_run
//! extern crate jmphash;
//!
//! use jmphash::jump_hash;
//!
//! fn really_complicated_hash_function() -> u64 {
//!   42
//! }
//!
//! fn main() {
//!     let num_buckets = 100;
//!
//!     // first use a hash function of your choice to create a u64 key
//!     let key = really_complicated_hash_function();
//!
//!     // then one can use `jump_hash` to map the key to a bucket
//!     let bucket = jump_hash(key, num_buckets);
//! }
//! ```

#![cfg_attr(feature = "nightly", feature(test))]

#[cfg(feature = "nightly")]
extern crate test;

pub fn jump_hash(mut key: u64, num_buckets: i64) -> i64 {
    let mut b = 0;
    let mut j = 0;
    while j < num_buckets {
        b = j;
        key = key.wrapping_mul(2862933555777941757).wrapping_add(1);
        j = ((b.wrapping_add(1) as f64) * ((1i64 << 31) as f64) /
             ((key >> 33).wrapping_add(1) as f64)) as i64;
    }

    b
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "nightly")]
    use test::Bencher;

    use super::jump_hash;

    #[test]
    fn jump_hash_test() {
        assert_eq!(jump_hash(123456, 1000), 984);
    }

    #[cfg(feature = "nightly")]
    #[bench]
    pub fn jump_hash_bench(b: &mut Bencher) {
        let mut i = 0;
        b.iter(|| {
            jump_hash(i, 1000);
            i += 1;
        })
    }
}
