//! Rust types and gRPC bindings generated from JOT protocol definitions.
//!
//! The module path omits the redundant `jot` package prefix because the crate
//! itself is named `jot_proto`.

pub mod judge {
    pub mod v1 {
        include!("generated/jot/judge/v1/jot.judge.v1.rs");
    }
}
