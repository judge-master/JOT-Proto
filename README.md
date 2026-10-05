# JOT-Proto

Versioned Protobuf contracts and language bindings for JOT.

- [`proto/judge/v1/judge.proto`](proto/judge/v1/judge.proto): judge service contract (`jot.judge.v1`).
- [`rust/`](rust/): `jot-proto` Rust crate with generated Protobuf types and Tonic gRPC bindings.
- [`go/`](go/): reserved for future Go bindings.

The Rust crate is versioned independently, with its major version matching the protocol major version. To regenerate Rust bindings after editing a protocol, follow the [Rust crate instructions](rust/README.md).

Licensed under [Apache-2.0](LICENSE).
