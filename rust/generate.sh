#!/bin/sh
set -eu

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$repo_root"
PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"
export PATH

mkdir -p rust/src/generated
protoc -I proto proto/judge/v1/judge.proto \
  --prost_out=rust/src/generated \
  --tonic_out=rust/src/generated
