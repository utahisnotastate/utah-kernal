#Requires -RunAsAdministrator
<#
.SYNOPSIS
  Utah-OS Universal Ghost-Boot USB creator (dad-proof one-click key).

.DESCRIPTION
  Formats a removable USB drive as FAT32, installs UEFI boot files, GRUB config,
  and the Utah-Kernel binary. Does NOT format your internal Windows drive.

.PARAMETER DriveLetter
  Optional drive letter (e.g. E). If omitted, a grid picker is shown.

.EXAMPLE
  .\tools\create_utah_usb.ps1
  .\tools\create_utah_usb.ps1 -DriveLetter E
#>

param(
    [string]$DriveLetter
)

$ErrorActionPreference = "Stop"
$RepoRoot = Split-Path -Parent $PSScriptRoot
$CoreDir = Join-Path $RepoRoot "core"
$GrubTemplate = Join-Path $PSScriptRoot "grub\utah_grub.cfg"

function Find-KernelBinary {
    $candidates = @(
        (Join-Path $CoreDir "target\x86_64-unknown-none\release\bootimage-utah-kernel.bin"),
        (Join-Path $CoreDir "target\x86_64-unknown-none\debug\bootimage-utah-kernel.bin")
    )
    foreach ($path in $candidates) {
        if (Test-Path $path) { return (Resolve-Path $path).Path }
    }
    return $null
}

function Select-RemovableVolume {
    param([string]$Letter)

    if ($Letter) {
        $vol = Get-Volume -DriveLetter $Letter -ErrorAction Stop
        if ($vol.DriveType -ne "Removable") {
            throw "Drive ${Letter}: is not removable. Refusing to format."
        }
        return $vol
    }

    $removable = Get-Volume | Where-Object {
        $_.DriveType -eq "Removable" -and $_.DriveLetter -and $_.FileSystem
    }
    if (-not $removable) {
        throw "No removable USB volumes detected. Insert a USB drive."
    }
    return $removable | Out-GridView -Title "Select USB drive for Utah-OS (ALL DATA WILL BE ERASED)" -PassThru
}

Write-Host "=========================================="
Write-Host " Utah-OS Ghost-Boot USB Creator"
Write-Host "=========================================="

if (-not ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole(
        [Security.Principal.WindowsBuiltInRole]::Administrator)) {
    throw "Run PowerShell as Administrator."
}

$target = Select-RemovableVolume -Letter $DriveLetter
$letter = $target.DriveLetter
Write-Host "[UTAH-USB] Target: ${letter}: ($($target.FileSystemLabel))"

$confirm = Read-Host "Format ${letter}: and install Utah-OS? Type YES to continue"
if ($confirm -ne "YES") {
    Write-Host "[CANCELLED] No changes made."
    exit 0
}

Write-Host "[UTAH-USB] Formatting ${letter}: as FAT32 UTAH-OS..."
Format-Volume -DriveLetter $letter -FileSystem FAT32 -NewFileSystemLabel "UTAH-OS" -Confirm:$false | Out-Null

$kernel = Find-KernelBinary
if (-not $kernel) {
    Write-Warning @"
Kernel binary not found. USB will get GRUB layout only.
Build first:  cd core; cargo bootimage --release
"@
}

# UEFI standard path
$efiBoot = "${letter}:\EFI\BOOT"
$efiUtah = "${letter}:\EFI\UtahOS\boot"
$grubDir = "${letter}:\boot\grub"
$utahDir = "${letter}:\UTAH"

New-Item -ItemType Directory -Force -Path $efiBoot, $efiUtah, $grubDir, $utahDir | Out-Null

if ($kernel) {
    Copy-Item -Path $kernel -Destination (Join-Path $efiBoot "BOOTX64.EFI") -Force
    Copy-Item -Path $kernel -Destination (Join-Path $efiUtah "BOOTX64.EFI") -Force
    Copy-Item -Path $kernel -Destination (Join-Path $utahDir "utah-kernel.bin") -Force
    Write-Host "[UTAH-USB] Kernel -> EFI\BOOT\BOOTX64.EFI"
}

if (Test-Path $GrubTemplate) {
    Copy-Item -Path $GrubTemplate -Destination (Join-Path $grubDir "grub.cfg") -Force
    Write-Host "[UTAH-USB] GRUB config -> boot\grub\grub.cfg"
} else {
    @"
set timeout=3
set default=0
menuentry 'Utah-OS Reality Console' {
  chainloader /EFI/BOOT/BOOTX64.EFI
}
menuentry 'Windows (return - remove USB)' {
  echo Boot internal drive from firmware after removing USB
}
"@ | Set-Content -Path (Join-Path $grubDir "grub.cfg") -Encoding ASCII
}

@"
Utah-OS Ghost-Boot USB
=====================
1. Reboot and press F12 / F2 / Del for boot menu
2. Select the USB drive labeled UTAH-OS
3. Utah-OS kernel initializes first (target <5ms path on bare metal)
4. Windows remains on internal disk - not erased by this USB tool

Hypervisor roadmap: see docs/GHOST_BOOT.md
"@ | Set-Content -Path "${letter}:\README.txt" -Encoding ASCII

Write-Host ""
Write-Host "[SUCCESS] Utah-OS manifested on USB ${letter}:"
Write-Host "  Reboot -> Boot from USB -> Utah-OS"
Write-Host "  Internal Windows/games partition untouched."
