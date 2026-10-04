#!/usr/bin/env bash
#
# Build xaml-lang-formatter release binaries and bundle them into the Weblate
# add-on package at src/xaml_lang_formatter_addon/bin/<platform-key>/.
#
# Usage:
#   scripts/build-binaries.sh                 # build for the current host
#   scripts/build-binaries.sh <triple>:<key>  # build an explicit target
#
# Examples:
#   scripts/build-binaries.sh x86_64-unknown-linux-musl:linux-x86_64
#   scripts/build-binaries.sh aarch64-unknown-linux-musl:linux-aarch64
#   scripts/build-binaries.sh x86_64-apple-darwin:darwin-x86_64

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
BIN_ROOT="$ROOT/contrib/weblate/src/xaml_lang_formatter_addon/bin"
BIN="xaml-lang-formatter"

key_for_target() {
  case "$1" in
    x86_64-unknown-linux-gnu | x86_64-unknown-linux-musl) echo "linux-x86_64" ;;
    aarch64-unknown-linux-gnu | aarch64-unknown-linux-musl) echo "linux-aarch64" ;;
    x86_64-apple-darwin) echo "darwin-x86_64" ;;
    aarch64-apple-darwin) echo "darwin-arm64" ;;
    x86_64-pc-windows-msvc | x86_64-pc-windows-gnu) echo "windows-x86_64" ;;
    *) return 1 ;;
  esac
}

build_target() {
  local target="$1" key="$2"
  echo "==> Building ${target} (${key})"

  rustup target add "${target}" >/dev/null 2>&1 || true
  cargo build --release --manifest-path "${ROOT}/Cargo.toml" --target "${target}"

  local dir="${ROOT}/target/${target}/release"
  local src="${dir}/${BIN}"
  local name="${BIN}"
  if [[ -f "${dir}/${BIN}.exe" ]]; then
    src="${dir}/${BIN}.exe"
    name="${BIN}.exe"
  fi

  if [[ ! -f "${src}" ]]; then
    echo "error: binary not found: ${src}" >&2
    exit 1
  fi

  mkdir -p "${BIN_ROOT}/${key}"
  cp "${src}" "${BIN_ROOT}/${key}/${name}"
  chmod +x "${BIN_ROOT}/${key}/${name}"
  echo "    bundled -> ${BIN_ROOT}/${key}/${name}"
}

if [[ "$#" -gt 0 ]]; then
  for spec in "$@"; do
    build_target "${spec%%:*}" "${spec##*:}"
  done
else
  host="$(rustc -vV | awk '/^host:/ {print $2}')"
  if ! key="$(key_for_target "${host}")"; then
    echo "error: unsupported host target: ${host}" >&2
    exit 1
  fi
  build_target "${host}" "${key}"
fi

echo
echo "Bundled binaries:"
find "${BIN_ROOT}" -type f ! -name '.gitignore' -print
