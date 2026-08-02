#!/bin/sh
# Installs the latest damem release into ~/.local/bin (override with DAMEM_BIN_DIR).
set -eu

repo=cunarist/damem
bin_dir=${DAMEM_BIN_DIR:-$HOME/.local/bin}

os=$(uname -s)
arch=$(uname -m)
case "$os $arch" in
  "Darwin arm64") target=aarch64-apple-darwin ;;
  "Darwin x86_64") target=x86_64-apple-darwin ;;
  "Linux x86_64") target=x86_64-unknown-linux-musl ;;
  "Linux aarch64" | "Linux arm64") target=aarch64-unknown-linux-gnu ;;
  *)
    echo "damem: no prebuilt binary for $os $arch" >&2
    echo "build it with: cargo install --git https://github.com/$repo" >&2
    exit 1
    ;;
esac

url="https://github.com/$repo/releases/latest/download/damem-$target.tar.gz"
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

echo "damem: downloading $target"
if command -v curl >/dev/null 2>&1; then
  curl -fsSL "$url" -o "$work/damem.tar.gz"
elif command -v wget >/dev/null 2>&1; then
  wget -qO "$work/damem.tar.gz" "$url"
else
  echo "damem: needs curl or wget" >&2
  exit 1
fi

tar xzf "$work/damem.tar.gz" -C "$work"
mkdir -p "$bin_dir"
mv "$work/damem-$target/damem" "$bin_dir/damem"
chmod +x "$bin_dir/damem"

echo "damem: installed to $bin_dir/damem"
case ":$PATH:" in
  *":$bin_dir:"*) ;;
  *) echo "damem: add it to your PATH with: export PATH=\"$bin_dir:\$PATH\"" ;;
esac
