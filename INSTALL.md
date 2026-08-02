# Installing damem

Run the block for your operating system. It installs one binary into a
per-user directory that is normally already on `PATH`, so nothing needs `sudo`
and one install serves every project on the machine.

## Linux

```shell
dir="$HOME/.local/bin"
mkdir -p "$dir"
t="$(uname -m)-unknown-linux-gnu"
curl -fsSL "https://github.com/cunarist/damem/releases/latest/download/damem-$t.tar.gz" \
  | tar xz -C "$dir" --strip-components=1 "damem-$t/damem"
case ":$PATH:" in *":$dir:"*) damem --version;; *) echo "Add $dir to PATH";; esac
```

## macOS

```shell
dir="$HOME/.local/bin"
mkdir -p "$dir"
t="$(uname -m)-apple-darwin"
curl -fsSL "https://github.com/cunarist/damem/releases/latest/download/damem-$t.tar.gz" \
  | tar xz -C "$dir" --strip-components=1 "damem-$t/damem"
case ":$PATH:" in *":$dir:"*) damem --version;; *) echo "Add $dir to PATH";; esac
```

## Windows

```powershell
$dir = "$env:LOCALAPPDATA\Microsoft\WindowsApps"
$t = if ($env:PROCESSOR_ARCHITECTURE -eq 'ARM64') { 'aarch64-pc-windows-msvc' } else { 'x86_64-pc-windows-msvc' }
$zip = Join-Path $env:TEMP "damem-$t.zip"
Invoke-WebRequest "https://github.com/cunarist/damem/releases/latest/download/damem-$t.zip" -OutFile $zip
Expand-Archive $zip -DestinationPath $env:TEMP -Force
Move-Item (Join-Path $env:TEMP "damem-$t\damem.exe") (Join-Path $dir "damem.exe") -Force
damem --version
```

`WindowsApps` is on `PATH` for every user account by default, so the command
works in the same terminal, without restarting anything.

## After installing

Run `damem recall` and treat its output as the context for the repository you
are working in.

## If `PATH` does not include the directory

The install still succeeded; the shell just cannot find it yet. Add the line to
your shell profile and open a new terminal:

```shell
export PATH="$HOME/.local/bin:$PATH"
```

Until then, call it by its full path: `~/.local/bin/damem recall`.

## Upgrading

Run the same block again. It overwrites the old binary.

## Notes

- `releases/latest/download/` always resolves to the newest release, so these
  commands need no editing when a version ships. To pin a version, replace
  `latest/download` with `download/v0.2.0`.
- Linux builds are glibc. On Alpine or another musl system, use
  `x86_64-unknown-linux-musl` in place of the `$t` above.
- Checksums are published next to each archive as `<archive>.sha256`.
- Downloading through a browser instead of the commands above marks the binary
  as quarantined on macOS. `curl` does not, which is why these use it.

