# JOT Protocol for Rust

This crate provides Protobuf message types and Tonic gRPC client/server bindings for the JOT protocols. The protocol definitions in `../proto/` are the source of truth; generated Rust files are committed so consumers do not need `protoc` during their builds.

## Use locally

```toml
[dependencies]
jot-proto = { path = "../JOT-Proto/rust" }
```

```rust
use jot_proto::judge::v1::{
    JudgeRequest,
    judge_service_client::JudgeServiceClient,
    judge_service_server::{JudgeService, JudgeServiceServer},
};
```

`JudgeService` is the generated server trait. Applications implement it and pass their implementation to `JudgeServiceServer::new`.

The crate's major version follows the protocol major version: `1.x` exposes `jot.judge.v1`. Minor and patch releases can evolve independently.

## Regenerate bindings

Install `protoc` and the matching generators:

```sh
cargo install protoc-gen-prost --version 0.5.0
cargo install protoc-gen-tonic --version 0.5.0
```

From the repository root, run:

```sh
./rust/generate.sh
cargo test --manifest-path rust/Cargo.toml
```

The script reads `proto/judge/v1/judge.proto` and writes to `rust/src/generated/`. Commit protocol and generated-code changes together.
