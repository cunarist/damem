# Installs the latest damem release into %LOCALAPPDATA%\Programs\damem.
$ErrorActionPreference = 'Stop'

$repo = 'cunarist/damem'
$target = switch ($env:PROCESSOR_ARCHITECTURE) {
  'AMD64' { 'x86_64-pc-windows-msvc' }
  'ARM64' { 'aarch64-pc-windows-msvc' }
  default { throw "damem: no prebuilt binary for $env:PROCESSOR_ARCHITECTURE" }
}

$binDir = if ($env:DAMEM_BIN_DIR) { $env:DAMEM_BIN_DIR } else { "$env:LOCALAPPDATA\Programs\damem" }
$work = Join-Path ([System.IO.Path]::GetTempPath()) "damem-install-$PID"
New-Item -ItemType Directory -Force $work | Out-Null

try {
  Write-Host "damem: downloading $target"
  $url = "https://github.com/$repo/releases/latest/download/damem-$target.zip"
  $zip = Join-Path $work 'damem.zip'
  Invoke-WebRequest -Uri $url -OutFile $zip -UseBasicParsing
  Expand-Archive -Path $zip -DestinationPath $work -Force

  New-Item -ItemType Directory -Force $binDir | Out-Null
  Copy-Item (Join-Path $work "damem-$target\damem.exe") $binDir -Force
} finally {
  Remove-Item -Recurse -Force $work -ErrorAction SilentlyContinue
}

Write-Host "damem: installed to $binDir\damem.exe"

# Put it on PATH for future sessions, and for this one.
$userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
if ($userPath -notlike "*$binDir*") {
  [Environment]::SetEnvironmentVariable('Path', "$userPath;$binDir", 'User')
  Write-Host 'damem: added to your PATH; open a new terminal to pick it up'
}
$env:Path = "$env:Path;$binDir"
