# Run a Release build
# Not sure I need the dual build for MacOS anymore
# if ($env:IsWindows) {
#     & cargo build --release
# }
# elseif ($env:IsMacOS) {
#     $appName = $env:ExeDcli
#     # Build for both x86_64 and aarch64
#     Invoke-Expression "cargo build --release --target aarch64-apple-darwin"
#     Invoke-Expression "cargo build --release --target x86_64-apple-darwin"

#     # Combine the two binaries into a universal binary
#     Invoke-Expression "lipo -create -output target/release/$appName target/x86_64-apple-darwin/release/$appName target/aarch64-apple-darwin/release/$appName"
# }

# Run a Release build
& cargo build --release

if ($LASTEXITCODE -eq 0) {
    # Init folders from environment variables
    $sourceApp = Join-Path -Path $env:LocalDev "dcli/target/release"
    $sourceApp = Join-Path -Path $sourceApp -ChildPath $env:ExeDcli
    $targetPath = $env:LocalDcli

    # Copy to local folder
    Copy-Item $sourceApp -Destination $targetPath -Force

    Write-Host "Build and Deploy Successful!"
}
else {
    Write-Host "Build failed! Could not Deploy."
}
