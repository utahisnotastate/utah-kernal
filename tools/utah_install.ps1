#Requires -RunAsAdministrator
<#
.SYNOPSIS
  Utah-OS Ghost-Burner: UEFI partition infiltrator for sovereign dual-boot with Windows.

.DESCRIPTION
  Creates or extends an EFI System Partition entry for Utah-OS, copies the forged
  kernel binary, and registers a Windows BCD boot menu item. Does NOT remove Windows.

.NOTES
  1. Build the kernel first:  cd core; cargo bootimage --release
  2. Reboot and pick "Utah-OS Reality Console" from the firmware/boot menu.
#>

$ErrorActionPreference = "Stop"

$RepoRoot = Split-Path -Parent $PSScriptRoot
$CoreDir = Join-Path $RepoRoot "core"

function Find-KernelBinary {
    $candidates = @(
        (Join-Path $CoreDir "target\x86_64-unknown-none\release\bootimage-utah-kernel.bin"),
        (Join-Path $CoreDir "target\x86_64-unknown-none\debug\bootimage-utah-kernel.bin"),
        (Join-Path $CoreDir "target\x86_64-unknown-none\release\utah-kernel"),
        (Join-Path $CoreDir "target\x86_64-unknown-none\debug\utah-kernel")
    )
    foreach ($path in $candidates) {
        if (Test-Path $path) { return (Resolve-Path $path).Path }
    }
    return $null
}

function Get-OrCreate-UtahEfiPartition {
    param([Microsoft.Management.Infrastructure.CimInstance]$BootDisk)

    $espType = "{c12a7328-f81f-11d2-ba4b-00a0c93ec93b}"
    $existingEsp = Get-Partition -DiskNumber $BootDisk.Number -ErrorAction SilentlyContinue |
        Where-Object { $_.GptType -eq $espType -or $_.Type -eq "System" } |
        Select-Object -First 1

    if ($existingEsp) {
        $letter = ($existingEsp | Get-Partition | Select-Object -First 1).DriveLetter
        if (-not $letter) {
            $letter = "Z"
            if (Get-PSDrive -Name $letter -ErrorAction SilentlyContinue) {
                throw "Cannot assign drive letter for EFI partition."
            }
            Set-Partition -DiskNumber $BootDisk.Number -PartitionNumber $existingEsp.PartitionNumber -NewDriveLetter $letter
        }
        Write-Host "[UTAH-INSTALL] Using existing EFI partition ($letter`:)"
        return @{ Disk = $BootDisk.Number; Partition = $existingEsp.PartitionNumber; Letter = $letter; Created = $false }
    }

    $unallocated = ($BootDisk | Get-Disk).AllocatedSize
    $diskSize = $BootDisk.Size
    if (($diskSize - $unallocated) -lt 600MB) {
        throw "No unallocated space for a 512MB Utah EFI partition. Free space or shrink a volume first."
    }

    Write-Host "[UTAH-INSTALL] Creating 512MB Utah EFI partition..."
    $newPart = New-Partition -DiskNumber $BootDisk.Number -Size 512MB -GptType $espType
    $letter = "U"
    while (Get-PSDrive -Name $letter -ErrorAction SilentlyContinue) { $letter = [char]([int][char]$letter + 1) }
    Format-Volume -Partition $newPart -FileSystem FAT32 -NewFileSystemLabel "UTAH-OS" -Confirm:$false | Out-Null
    Set-Partition -DiskNumber $BootDisk.Number -PartitionNumber $newPart.PartitionNumber -NewDriveLetter $letter
    return @{ Disk = $BootDisk.Number; Partition = $newPart.PartitionNumber; Letter = $letter; Created = $true }
}

function Install-UtahEfiFiles {
    param([string]$DriveLetter, [string]$KernelPath)

    $efiRoot = "${DriveLetter}:\EFI\UtahOS"
    $bootDir = Join-Path $efiRoot "boot"
    New-Item -ItemType Directory -Force -Path $bootDir | Out-Null

    $dest = Join-Path $bootDir "BOOTX64.EFI"
    Copy-Item -Path $KernelPath -Destination $dest -Force
    Write-Host "[UTAH-INSTALL] Installed kernel -> $dest"

    $readme = @"
Utah-OS Ghost-Burner
Kernel: BOOTX64.EFI
Repo: https://github.com/utahisnotastate/utah-kernal
"@
    Set-Content -Path (Join-Path $efiRoot "README.txt") -Value $readme -Encoding ASCII
    return $dest
}

function Register-BcdUtahEntry {
    param([string]$EfiDevicePath)

    $guid = [guid]::NewGuid().ToString("B")
    $displayName = "Utah-OS Reality Console"

    $existing = bcdedit /enum firmware 2>&1 | Out-String
    if ($existing -match "Utah-OS") {
        Write-Host "[UTAH-INSTALL] BCD entry may already exist; skipping duplicate."
        return
    }

    bcdedit /copy "{bootmgr}" /d $displayName | Out-Null
    bcdedit /set "{default}" description $displayName 2>$null | Out-Null

    Write-Host "[UTAH-INSTALL] Registering UEFI boot application..."
    $createOut = bcdedit /create $guid /d $displayName /application bootsector 2>&1
    if ($LASTEXITCODE -ne 0) {
        Write-Warning "bootsector entry failed ($createOut). Trying firmware device path..."
    }

    $fwGuid = [guid]::NewGuid().ToString("B")
    bcdedit /create $fwGuid /d $displayName /device 2>&1 | Out-Null
    bcdedit /set $fwGuid path $EfiDevicePath 2>&1 | Out-Null
    bcdedit /set $fwGuid description $displayName 2>&1 | Out-Null
    bcdedit /bootsequence $fwGuid /addfirst 2>&1 | Out-Null

    Write-Host "[UTAH-INSTALL] BCD configured. GUID hints: $guid / $fwGuid"
}

# --- Main ---
Write-Host "=========================================="
Write-Host " Utah-OS Ghost-Burner (UEFI Infiltrator)"
Write-Host "=========================================="

if (-not ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole(
        [Security.Principal.WindowsBuiltInRole]::Administrator)) {
    throw "Run PowerShell as Administrator."
}

$kernel = Find-KernelBinary
if (-not $kernel) {
    throw @"
Kernel binary not found. Build first:
  cd `"$CoreDir`"
  cargo bootimage --release
"@
}
Write-Host "[UTAH-INSTALL] Kernel artifact: $kernel"

$bootDisk = Get-Disk | Where-Object { $_.IsBoot -eq $true } | Select-Object -First 1
if (-not $bootDisk) { throw "No boot disk detected." }

$esp = Get-OrCreate-UtahEfiPartition -BootDisk $bootDisk
$efiPath = Install-UtahEfiFiles -DriveLetter $esp.Letter -KernelPath $kernel

$devicePath = "\EFI\UtahOS\boot\BOOTX64.EFI"
Register-BcdUtahEntry -EfiDevicePath $devicePath

Write-Host ""
Write-Host "[SUCCESS] Utah-OS injected into EFI boot chain."
Write-Host "  Partition: $($esp.Letter): (created=$($esp.Created))"
Write-Host "  Payload:   $efiPath"
Write-Host "  Reboot and select 'Utah-OS Reality Console' to ascend."
Write-Host ""
Write-Host "Windows and your games remain on their existing volumes."
