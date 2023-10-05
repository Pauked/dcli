$buildCmd = "cargo build --release"

# Build the Rust project
Invoke-Expression $buildCmd

# Extract version number and name from Cargo.toml
$cargoTomlContent = Get-Content -Path "Cargo.toml"
$versionLine = $cargoTomlContent | Where-Object { $_ -like "version =*" }
$nameLine = $cargoTomlContent | Where-Object { $_ -like "name =*" }

$version = ($versionLine -split '=')[1].Trim().Trim('"')
$appName = ($nameLine -split '=')[1].Trim().Trim('"')

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

# Determine OS and prepare OS-specific variables
$osInfo = ""
if ($env:IsWindows) {
    $appBinary = "$releaseDir/$appName.exe"
    $osInfo = "win64"
} elseif ($env:IsMacOS) {
    $appBinary = "$releaseDir/$appName"
    $osInfo = "macOS"
} else {
    Write-Output "Unsupported OS"
    exit
}

# Append OS info to compressed file name
$compressedFileName = "$appName-v$version-$osInfo"
$archivePath = "$releaseDir/$compressedFileName.zip"

# Check if the zip file exists, and if so, delete it
if (Test-Path $archivePath) {
    Remove-Item $archivePath
}

# Adding application binary to files to include
$filesToInclude += $appBinary

# Create a temporary directory
$tempDir = "$releaseDir/temp"

# Ensure the temporary directory is clean
if (Test-Path $tempDir) {
    Remove-Item -Recurse -Force $tempDir
}

New-Item -ItemType Directory -Force -Path $tempDir
CopyFilesToTemp -files $filesToInclude -tempDir $tempDir -appName $appName

# Compress the files into a .zip for both OS types
Compress-Archive -Path "$tempDir/*" -DestinationPath $archivePath

# Clean up the temporary directory
Remove-Item -Recurse -Force $tempDir
