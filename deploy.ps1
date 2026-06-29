param([Parameter(ValueFromRemainingArguments=$true)][string[]]$ProgArgs)

$ErrorActionPreference = "Stop"

# --- Load .env from the script's own directory ---
$EnvFile = Join-Path $PSScriptRoot ".env"
if (-not (Test-Path $EnvFile)) {
    Write-Error ".env not found at $EnvFile (copy .env.example to .env)"
}

$config = @{}
foreach ($line in Get-Content $EnvFile) {
    $trimmed = $line.Trim()
    if ($trimmed -eq "" -or $trimmed.StartsWith("#")) { continue }
    $idx = $trimmed.IndexOf("=")
    if ($idx -lt 1) { continue }
    $key = $trimmed.Substring(0, $idx).Trim()
    $val = $trimmed.Substring($idx + 1).Trim().Trim('"')
    $config[$key] = $val
}

$Pi      = $config["PI_HOST"]
$DestDir = $config["PI_DEST_DIR"]
if (-not $Pi)      { Write-Error "PI_HOST not set in .env" }
if (-not $DestDir) { Write-Error "PI_DEST_DIR not set in .env" }

# --- Build for aarch64 ---
Write-Host "→ Building for aarch64..." -ForegroundColor Cyan
cross build --target aarch64-unknown-linux-gnu
if ($LASTEXITCODE -ne 0) { Write-Error "Build failed" }

# --- Locate binary (package name = Spooder) ---
$Bin = Join-Path $PSScriptRoot "target\aarch64-unknown-linux-gnu\debug\Spooder"
if (-not (Test-Path $Bin)) { Write-Error "Binary not found at $Bin" }

$Name = Split-Path $Bin -Leaf
$Dest = "$DestDir/$Name"

# --- Deploy and run ---
Write-Host "→ Copying $Name to $Pi..." -ForegroundColor Cyan
scp $Bin "${Pi}:${Dest}"
if ($LASTEXITCODE -ne 0) { Write-Error "scp failed" }

Write-Host "→ Running on Pi..." -ForegroundColor Cyan
ssh $Pi "chmod +x $Dest && $Dest $ProgArgs"