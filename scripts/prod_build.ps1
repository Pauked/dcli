# [Common Function Definitions]
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
        }
        else {
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

function CleanupAndExit() {
    Remove-Item -Recurse -Force $tempDir
    Remove-Item -Path "readme.html" -Force
    exit
}

# [Pre-build Tasks]
$filesToInclude = @(
    "docs/dcli-full-main-menu.png",
    "docs/dcli-simple-main-menu.png",
    "scripts/test_macos.sh",
    "scripts/test_windows.ps1",
    "scripts/test_windows.bat",
    "readme.html"
)

$versionLine = (Get-Content -Path "Cargo.toml") | Where-Object { $_ -like "version =*" }
$nameLine = (Get-Content -Path "Cargo.toml") | Where-Object { $_ -like "name =*" }

$version = ($versionLine -split '=')[1].Trim().Trim('"')
$appName = ($nameLine -split '=')[1].Trim().Trim('"')

# Convert readme.md to readme.html using mangler
$manglerCmd = "$env:LocalBuildTools\$env:ExeMangler readme.md readme.html 'dcli readme'"
Invoke-Expression $manglerCmd

# [Build Process]
if ($env:IsWindows) {
    Invoke-Expression "cargo build --release"
    $appBinary = "target/release/$appName.exe"
    $osInfo = "win64"
    $compressionFormat = "zip"
}
elseif ($env:IsMacOS) {
    # Build for both x86_64 and aarch64
    Invoke-Expression "cargo build --release --target aarch64-apple-darwin"
    Invoke-Expression "cargo build --release --target x86_64-apple-darwin"

    # Combine the two binaries into a universal binary
    Invoke-Expression "lipo -create -output target/release/$appName target/x86_64-apple-darwin/release/$appName target/aarch64-apple-darwin/release/$appName"

    $appBinary = "target/release/$appName"
    $osInfo = "macOS"
    $compressionFormat = "dmg"
}
else {
    Write-Output "Unsupported OS"
    exit
}

# [Packaging Process]
# Prepare archive name
$compressedFileName = "$appName-v$version-$osInfo"
$releaseDir = "target/release"
$archivePath = "$releaseDir/$compressedFileName.$compressionFormat"

# Delete if the archive already exists
if (Test-Path $archivePath) { Remove-Item $archivePath }

# Add the binary, which differs depending on the OS, to the files to include list
$filesToInclude += $appBinary
$tempDir = "$releaseDir/temp"

# Ensure temp directory is empty and exists
if (Test-Path $tempDir) { Remove-Item -Recurse -Force $tempDir }
New-Item -ItemType Directory -Force -Path $tempDir

# Copy files to temp directory
CopyFilesToTemp -files $filesToInclude -tempDir $tempDir -appName $appName

if ($env:IsWindows) {
    # Create ZIP using Compress-Archive
    Compress-Archive -Path "$tempDir/*" -DestinationPath $archivePath
}
elseif ($env:IsMacOS) {
    # Check if the DMG file exists, and if so, delete it
    if (Test-Path $archivePath) {
        Remove-Item $archivePath
    }

    # Create DMG using hdiutil
    Invoke-Expression "hdiutil create -srcfolder $tempDir -volname $appName -format UDZO -fs HFS+ -o $archivePath"
}

# [Copy to Release Folder]
$releaseDir = $env:ReleaseFolderDcli
New-Item -ItemType Directory -Force -Path $releaseDir
Copy-Item -Path $archivePath -Destination $releaseDir

# [Cleanup]
CleanupAndExit
