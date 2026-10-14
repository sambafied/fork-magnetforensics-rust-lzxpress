#!/usr/bin/env bash
# Build the GitHub release assets for one sambafied/main commit.
# The tag names the release. xpress_rs is consumed from that git tag.
set -euo pipefail

if [[ $# -ne 2 ]]; then
  echo "usage: package-release.sh <sambafied/vX.Y.Z> <output-directory>" >&2
  exit 1
fi

tag="$1"
output="$2"
if [[ ! "$tag" =~ ^sambafied/v[0-9]+\.[0-9]+\.[0-9]+-[0-9a-f]+$ ]]; then
  echo "tag must look like sambafied/v0.2.0-0123456789ab" >&2
  exit 1
fi

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
mkdir -p "$output"
rm -f "$output"/*.crate "$output"/SHA256SUMS

lib_version="$(sed -n 's/^version = "\(.*\)"/\1/p' "$root/Cargo.toml" | head -n 1)"

(
  cd "$root"
  cargo package --allow-dirty -p rust-lzxpress
)
cp "$root/target/package/rust-lzxpress-${lib_version}.crate" "$output/"

(
  cd "$output"
  sha256sum -- *.crate > SHA256SUMS
)
echo "packaged rust-lzxpress ${lib_version} for ${tag}"
