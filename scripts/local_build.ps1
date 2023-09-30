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
