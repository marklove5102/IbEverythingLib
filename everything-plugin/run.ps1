$EverythingDir = Join-Path $PSScriptRoot '../tests/everything/v1.5.0.1393_x64'
$TargetDir = Join-Path $PSScriptRoot '../target/debug/examples'

# Get-Process | Where-Object {$_.Path -eq '$EverythingDir\Everything64.exe'} | Stop-Process
cargo build --example test
if (!$?) {
    Write-Host "Build failed"
    exit 1
}
cargo build --example basic
if (!$?) {
    Write-Host "Build failed"
    exit 1
}
cargo build --example options
if (!$?) {
    Write-Host "Build failed"
    exit 1
}

# Create symbolic links for the DLLs to Everything directory
$plugins = "$EverythingDir\plugins"
if (Test-Path "$plugins\test.dll") { Remove-Item "$plugins\test.dll" }
if (Test-Path "$plugins\basic.dll") { Remove-Item "$plugins\basic.dll" }
if (Test-Path "$plugins\options.dll") { Remove-Item "$plugins\options.dll" }
New-Item -Path "$plugins\test.dll" -ItemType SymbolicLink -Value "$TargetDir\test.dll" | Out-Null
New-Item -Path "$plugins\basic.dll" -ItemType SymbolicLink -Value "$TargetDir\basic.dll" | Out-Null
New-Item -Path "$plugins\options.dll" -ItemType SymbolicLink -Value "$TargetDir\options.dll" | Out-Null

$env:RUST_BACKTRACE='full'
& "$EverythingDir/Everything.exe" -debug
