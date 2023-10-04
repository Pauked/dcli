$buildCmd = "cargo build --release"

# Build the Rust project
Invoke-Expression $buildCmd

# Extract version number and name from Cargo.toml
$cargoTomlContent = Get-Content -Path "Cargo.toml"
$versionLine = $cargoTomlContent | Where-Object { $_ -like "version =*" }
$nameLine = $cargoTomlContent | Where-Object { $_ -like "name =*" }

$version = ($versionLine -split '=')[1].Trim().Trim('"')
$appName = ($nameLine -split '=')[1].Trim().Trim('"')

# Output file name
$compressedFileName = "$appName-v$version"

# Where the build will be
$releaseDir = "target/release"

# Determine OS and compress accordingly
if ($env:IsWindows) {
    # List of files to be compressed/archived for Windows
    $filesToInclude = @(
        "$releaseDir/$appName.exe",
        # "readme.txt",
        "readme.md"
    )

    # Check if the zip file exists, and if so, delete it
    $zipPath = "$releaseDir/$compressedFileName.zip"
    if (Test-Path $zipPath) {
        Remove-Item $zipPath
    }

    # Use Windows compression for .zip
    Compress-Archive -Path $filesToInclude -DestinationPath $zipPath
} elseif ($env:IsMacOS) {
    # List of files to be compressed/archived for macOS
    $filesToInclude = @(
        "$releaseDir/$appName",
        # "readme.txt",
        "readme.md"
    )

    # Check if the dmg file exists, and if so, delete it
    $dmgPath = "$releaseDir/$compressedFileName.dmg"
    if (Test-Path $dmgPath) {
        Remove-Item $dmgPath
    }

    # Create a temporary directory to hold the files before building the dmg
    $tempDir = "$releaseDir/temp"
    New-Item -ItemType Directory -Force -Path $tempDir
    foreach ($file in $filesToInclude) {
        Copy-Item -Path $file -Destination $tempDir
    }

    # Build a .dmg for macOS
    $dmgCmd = "hdiutil create $dmgPath -volname $appName -srcfolder $tempDir"
    Invoke-Expression $dmgCmd

    # Clean up the temporary directory
    Remove-Item -Recurse -Force $tempDir
} else {
    Write-Output "Unsupported OS"
}
