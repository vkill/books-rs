//! flatbuffer builder pool benchmark
//!
//! # Examples
//!
//! ```sh
//! $ c bench --bench pool
//! Finished bench [optimized] target(s) in 0.02s
//!
//! running 8 tests
//! test pool_global_v1 ... bench:          86 ns/iter (+/- 18)
//! test pool_global_v2 ... bench:         107 ns/iter (+/- 26)
//! test pool_global_v3 ... bench:          89 ns/iter (+/- 11)
//! test pool_local_v1  ... bench:         133 ns/iter (+/- 11)
//! test pool_local_v2  ... bench:         132 ns/iter (+/- 1)
//! test pool_local_v3  ... bench:         123 ns/iter (+/- 15)
//! test pool_mutex     ... bench:          44 ns/iter (+/- 1)
//! test pool_stack     ... bench:          60 ns/iter (+/- 15)
//!
//! test result: ok. 0 passed; 0 failed; 0 ignored; 8 measured; 0 filtered out
//! ```
#![feature(test)]
extern crate test;

use test::Bencher;

use flatbuf_tutorial::pool::{v1, v2, v3};
use flatbuffers::FlatBufferBuilder;
use parking_lot::Mutex;

const INIT_POOL_SIZE: usize = 4_096;
const MAX_POOL_SIZE: usize = 8_192;
const BUFFER_CAPACITY: usize = 64;

#[bench]
fn pool_stack(b: &mut Bencher) {
    b.iter(|| {
        let mut b = FlatBufferBuilder::new_with_capacity(BUFFER_CAPACITY);
        let data = b.create_string("a");
        b.finish(data, None);
    });
}

#[bench]
fn pool_mutex(b: &mut Bencher) {
    let builder = Mutex::new(FlatBufferBuilder::new_with_capacity(BUFFER_CAPACITY));
    b.iter(|| {
        let b = &mut *builder.lock();
        let data = b.create_string("a");
        b.finish(data, None);
    });
}

#[bench]
fn pool_global_v1(b: &mut Bencher) {
    v1::FlatBufferBuilderPool::init_global_pool_size(INIT_POOL_SIZE);
    v1::FlatBufferBuilderPool::max_global_pool_size(MAX_POOL_SIZE);
    v1::FlatBufferBuilderPool::global_buffer_capacity(BUFFER_CAPACITY);
    b.iter(|| {
        let mut b = v1::FlatBufferBuilderPool::get();
        let data = b.create_string("a");
        b.finish(data, None);
    });
}

#[bench]
fn pool_global_v2(b: &mut Bencher) {
    v2::FlatBufferBuilderPool::init_global_pool_size(INIT_POOL_SIZE);
    v2::FlatBufferBuilderPool::max_global_pool_size(MAX_POOL_SIZE);
    v2::FlatBufferBuilderPool::global_buffer_capacity(BUFFER_CAPACITY);
    b.iter(|| {
        let mut b = v2::FlatBufferBuilderPool::get();
        let data = b.create_string("a");
        b.finish(data, None);
    });
}

#[bench]
fn pool_global_v3(b: &mut Bencher) {
    v3::FlatBufferBuilderPool::init_global_pool_size(INIT_POOL_SIZE);
    v3::FlatBufferBuilderPool::max_global_pool_size(MAX_POOL_SIZE);
    v3::FlatBufferBuilderPool::global_buffer_capacity(BUFFER_CAPACITY);
    b.iter(|| {
        let mut b = v3::FlatBufferBuilderPool::get();
        let data = b.create_string("a");
        b.finish(data, None);
    });
}

#[bench]
fn pool_local_v1(b: &mut Bencher) {
    let pool = v1::FlatBufferBuilderPool::new()
        .init_pool_size(INIT_POOL_SIZE)
        .max_pool_size(MAX_POOL_SIZE)
        .buffer_capacity(BUFFER_CAPACITY)
        .build();
    b.iter(|| {
        let mut b = pool.get();
        let data = b.create_string("a");
        b.finish(data, None);
    });
}

#[bench]
fn pool_local_v2(b: &mut Bencher) {
    let pool = v2::FlatBufferBuilderPool::new()
        .init_pool_size(INIT_POOL_SIZE)
        .max_pool_size(MAX_POOL_SIZE)
        .buffer_capacity(BUFFER_CAPACITY)
        .build();
    b.iter(|| {
        let mut b = pool.get();
        let data = b.create_string("a");
        b.finish(data, None);
    });
}

#[bench]
fn pool_local_v3(b: &mut Bencher) {
    let pool = v3::FlatBufferBuilderPool::new()
        .init_pool_size(INIT_POOL_SIZE)
        .max_pool_size(MAX_POOL_SIZE)
        .buffer_capacity(BUFFER_CAPACITY)
        .build();
    b.iter(|| {
        let mut b = pool.get();
        let data = b.create_string("a");
        b.finish(data, None);
    });
}
