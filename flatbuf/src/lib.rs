//! [Flatbuffers tutorial](https://google.github.io/flatbuffers/flatbuffers_guide_tutorial.html).
pub mod model;
pub mod monster;
pub mod pool;
pub use monster::Monster;
pub use pool::{FlatBufferBuilderLocalPool, FlatBufferBuilderPool};
