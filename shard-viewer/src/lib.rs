//! shard-viewer: a graphical navigator for shard source.
//!
//! The library half is the headless model: a self-contained s-expr reader
//! (`sexpr`) and a structural project model with a call graph (`model`). The
//! GUI (damascene) is built on top in the `shard-viewer` binary.

pub mod layout;
pub mod model;
pub mod sexpr;
pub mod view;
