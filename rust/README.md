# JOT Protocol for Rust

Protobuf message types and Tonic gRPC bindings for JOT protocols. The [protocol definitions](https://github.com/judge-master/JOT-Proto/tree/main/proto) are the source of truth. Generated Rust code is included in this crate, so consumers do not need `protoc` to build it.

## Use

```toml
[dependencies]
jot-proto = "1"
```

```rust
use jot_proto::judge::v1::{
    JudgeRequest,
    judge_service_client::JudgeServiceClient,
    judge_service_server::{JudgeService, JudgeServiceServer},
};
```

`JudgeService` is the generated server trait. Implement it in your application and pass the implementation to `JudgeServiceServer::new`.

The crate's major version follows the protocol major version: `1.x` exposes `jot.judge.v1`. Minor and patch releases can evolve independently.

## Regenerate bindings from the repository

In a checkout of [JOT-Proto](https://github.com/judge-master/JOT-Proto), install `protoc` and the matching generators:

```sh
cargo install protoc-gen-prost --version 0.5.0
cargo install protoc-gen-tonic --version 0.5.0
```

From the repository root, run:

```sh
./rust/generate.sh
cargo test --manifest-path rust/Cargo.toml
```

Commit changes to the protocol definitions and generated Rust code together. The generator script and `.proto` sources live in the repository rather than the published Rust crate.

## License

Apache-2.0. See [LICENSE](LICENSE).
