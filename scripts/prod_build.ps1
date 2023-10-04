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

# Common files to be compressed/archived
$filesToInclude = @(
    "docs/dcli-full-main-menu.png",
    "docs/dcli-simple-main-menu.png",
    "scripts/test_macos.sh",
    "scripts/test_windows.ps1",
    "scripts/test_windows.bat",
    "readme.md"
)

function CopyFilesToTemp($files, $tempDir, $appName) {
    foreach ($file in $files) {
        # Determine destination path
        if ($file -match "$appName(\.exe)?$") {
            # If the file is the application binary, place it directly in $tempDir
            $dest = "$tempDir/$appName"
            if ($file -match "\.exe$") {
                # Append .exe if it's a Windows binary
                $dest += ".exe"
            }
        } else {
            # Otherwise, maintain the original folder structure
            $dest = "$tempDir/$file"
        }

        # Ensure destination directory exists
        $destDir = [System.IO.Path]::GetDirectoryName($dest)
        New-Item -ItemType Directory -Force -Path $destDir

        # Copy the file
        Copy-Item -Path $file -Destination $dest
    }
}

# Determine OS and compress accordingly
if ($env:IsWindows) {
    $appBinary = "$releaseDir/$appName.exe"
    $archivePath = "$releaseDir/$compressedFileName.zip"
    $filesToInclude += $appBinary

    # Check if the zip file exists, and if so, delete it
    if (Test-Path $archivePath) {
        Remove-Item $archivePath
    }

    # Create a temporary directory
    $tempDir = "$releaseDir/tempWindows"

} elseif ($env:IsMacOS) {
    $appBinary = "$releaseDir/$appName"
    $archivePath = "$releaseDir/$compressedFileName.dmg"
    $filesToInclude += $appBinary

    # Check if the dmg file exists, and if so, delete it
    if (Test-Path $archivePath) {
        Remove-Item $archivePath
    }

    # Create a temporary directory
    $tempDir = "$releaseDir/tempMacOS"

} else {
    Write-Output "Unsupported OS"
    exit
}

# Ensure the temporary directory is clean
if (Test-Path $tempDir) {
    Remove-Item -Recurse -Force $tempDir
}

New-Item -ItemType Directory -Force -Path $tempDir
CopyFilesToTemp -files $filesToInclude -tempDir $tempDir -appName $appName

# OS-specific compression logic
if ($env:IsWindows) {
    Compress-Archive -Path "$tempDir/*" -DestinationPath $archivePath
} elseif ($env:IsMacOS) {
    $dmgCmd = "hdiutil create $archivePath -volname $appName -srcfolder $tempDir"
    Invoke-Expression $dmgCmd
}

# Clean up the temporary directory
Remove-Item -Recurse -Force $tempDir