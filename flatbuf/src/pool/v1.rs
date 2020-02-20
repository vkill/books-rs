//! flatbuffer builder pool
use std::ops::{Deref, DerefMut};

use flatbuffers::FlatBufferBuilder;
use once_cell::sync::Lazy;
use parking_lot::Mutex;

const INIT_POOL_SIZE: usize = 32;
const MAX_POOL_SIZE: usize = 1_024;
const BUFFER_CAPACITY: usize = 64;

/// `FlatBufferBuilder` pool.
///
/// # Examples
///
/// ```
/// use flatbuf_tutorial::pool::v1::BuilderPool;
///
/// let mut b = BuilderPool::get();
/// let name = b.create_string("something fun");
/// b.finish(name, None);
/// ```
pub struct BuilderPool;

impl BuilderPool {
    /// get() returns the pre-allocated [`Builder`] from
    /// the pool, or returns the newly allocated one.
    ///
    /// [`builder`]: struct.Builder.html
    #[inline]
    pub fn get() -> Builder {
        let mut pool = POOL.lock();
        match pool.pop() {
            Some(builder) => builder,
            None => Builder::new(),
        }
    }
}

/// `Builder` encapsulates the `FlatBufferBuilder` instance.
pub struct Builder(Option<FlatBufferBuilder<'static>>);

impl Builder {
    #[inline]
    fn new() -> Self {
        Self::default()
    }
}

impl Default for Builder {
    #[inline]
    fn default() -> Self {
        Self(Some(FlatBufferBuilder::new_with_capacity(BUFFER_CAPACITY)))
    }
}

impl Deref for Builder {
    type Target = FlatBufferBuilder<'static>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}

impl DerefMut for Builder {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unwrap()
    }
}

impl Drop for Builder {
    #[inline]
    fn drop(&mut self) {
        if let Some(mut builder) = self.0.take() {
            // resetting the builder outside of the lock
            // to reduce the pool manipulation contention.
            builder.reset();
            let mut pool = POOL.lock();
            if pool.len() < MAX_POOL_SIZE {
                pool.push(Builder(Some(builder)))
            }
        }
    }
}

static POOL: Lazy<Mutex<Vec<Builder>>> = Lazy::new(|| {
    let mut pool = Vec::new();
    for _ in { 0..INIT_POOL_SIZE } {
        pool.push(Builder::new());
    }
    Mutex::new(pool)
});
