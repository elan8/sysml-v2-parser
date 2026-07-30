param(
    [string]$Version = "",
    [string]$Destination = "sysml-v2-release"
)

$ErrorActionPreference = "Stop"

function Resolve-ProjectPath([string]$PathValue) {
    if ([System.IO.Path]::IsPathRooted($PathValue)) {
        return [System.IO.Path]::GetFullPath($PathValue)
    }

    return [System.IO.Path]::GetFullPath((Join-Path $PSScriptRoot "..\$PathValue"))
}

function Read-ConformanceTargetValue([string]$TargetFile, [string]$Key) {
    $line = Get-Content -LiteralPath $TargetFile |
        Where-Object { $_ -match "^$Key=" } |
        Select-Object -First 1
    if (-not $line) {
        throw "Missing $Key in $TargetFile"
    }
    return ($line -split "=", 2)[1].Trim()
}

function Assert-ReleaseLayout([string]$Root) {
    $required = @(
        (Join-Path $Root "sysml"),
        (Join-Path $Root "sysml\src\validation"),
        (Join-Path $Root "sysml.library")
    )

    foreach ($path in $required) {
        if (-not (Test-Path -LiteralPath $path)) {
            throw "Downloaded archive is missing expected path: $path"
        }
    }
}

$targetFile = Resolve-ProjectPath "docs\conformance-target"
if (-not (Test-Path -LiteralPath $targetFile)) {
    throw "Conformance target file not found: $targetFile"
}

if ([string]::IsNullOrWhiteSpace($Version)) {
    $Version = Read-ConformanceTargetValue $targetFile "release_tag"
}

$destinationPath = Resolve-ProjectPath $Destination
$parentDir = Split-Path -Parent $destinationPath
$repoName = "SysML-v2-Release"
$archiveUrl = "https://github.com/Systems-Modeling/$repoName/archive/refs/tags/$Version.zip"

New-Item -ItemType Directory -Force -Path $parentDir | Out-Null

$tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ("sysml-v2-release-" + [System.Guid]::NewGuid().ToString("N"))
$archivePath = Join-Path $tempRoot "$Version.zip"
$extractDir = Join-Path $tempRoot "extract"
$expandedRoot = Join-Path $extractDir "$repoName-$Version"
$stagingPath = Join-Path $parentDir ("sysml-v2-release-stage-" + [System.Guid]::NewGuid().ToString("N"))

try {
    New-Item -ItemType Directory -Force -Path $tempRoot | Out-Null
    New-Item -ItemType Directory -Force -Path $extractDir | Out-Null

    Write-Host "Downloading $repoName $Version from $archiveUrl"
    Write-Host "(pinned by docs/conformance-target unless overridden)"
    Invoke-WebRequest -Uri $archiveUrl -OutFile $archivePath

    Write-Host "Extracting archive to temporary directory"
    Expand-Archive -LiteralPath $archivePath -DestinationPath $extractDir -Force

    if (-not (Test-Path -LiteralPath $expandedRoot)) {
        throw "Expected extracted directory not found: $expandedRoot"
    }

    Assert-ReleaseLayout $expandedRoot

    if (Test-Path -LiteralPath $stagingPath) {
        Remove-Item -LiteralPath $stagingPath -Recurse -Force
    }

    Move-Item -LiteralPath $expandedRoot -Destination $stagingPath

    # Stamp so CI/tests can prove the tree matches the conformance pin.
    @(
        "# Written by scripts/fetch-sysml-v2-release.ps1 — do not edit by hand."
        "release_tag=$Version"
    ) | Set-Content -LiteralPath (Join-Path $stagingPath ".elan8-conformance-target") -Encoding utf8

    if (Test-Path -LiteralPath $destinationPath) {
        Write-Host "Removing existing destination $destinationPath"
        Remove-Item -LiteralPath $destinationPath -Recurse -Force
    }

    Move-Item -LiteralPath $stagingPath -Destination $destinationPath
    Write-Host "SysML v2 release $Version is ready at $destinationPath"
} finally {
    if (Test-Path -LiteralPath $stagingPath) {
        Remove-Item -LiteralPath $stagingPath -Recurse -Force
    }
    if (Test-Path -LiteralPath $tempRoot) {
        Remove-Item -LiteralPath $tempRoot -Recurse -Force
    }
}
