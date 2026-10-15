# Trust the Drafting certificate and install/update the package.
# Run via install.bat (double-click) or from PowerShell.
$ErrorActionPreference = 'Stop'
$here = Split-Path -Parent $MyInvocation.MyCommand.Path

$admin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()
         ).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $admin) {
    Start-Process powershell -Verb RunAs -ArgumentList "-NoProfile -ExecutionPolicy Bypass -File `"$($MyInvocation.MyCommand.Path)`""
    exit
}

try {
    Import-Certificate -FilePath (Join-Path $here 'drafting-cert.cer') `
        -CertStoreLocation Cert:\LocalMachine\TrustedPeople | Out-Null
    Add-AppxPackage -Path (Join-Path $here 'drafting.msix') -ForceUpdateFromAnyVersion
    Write-Host 'Drafting installed.' -ForegroundColor Green
} catch {
    Write-Host "Install failed: $_" -ForegroundColor Red
}
Read-Host 'Press Enter to close'
