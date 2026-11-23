#!/bin/sh
# Builds every asset of an AnkiQuest-Art release into <out-dir>: exports
# both clients' art with tools/art-export, then packs it with bundle.py.
# Run from the repository root; <out-dir> must not exist yet.
#
# Usage: tools/release/build.sh <tag> <commit> <out-dir>
set -eu

if [ $# -ne 3 ]; then
  echo "usage: tools/release/build.sh <tag> <commit> <out-dir>" >&2
  exit 2
fi
tag=$1
commit=$2
out=$3
if [ -e "$out" ]; then
  echo "$out already exists" >&2
  exit 2
fi

work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
export_art() {
  cargo run --quiet --locked --release --manifest-path tools/art-export/Cargo.toml -- "$@" >/dev/null
}
export_art android "$work/res"
export_art addon "$work/addon"
python3 tools/release/bundle.py "$tag" "$commit" "$work/res" "$work/addon" "$out"
