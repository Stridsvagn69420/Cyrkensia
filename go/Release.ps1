#!/usr/bin/pwsh
$rawArray = $(go tool dist list).Split([Environment]::NewLine)
$releaseText = ""
foreach ($osArch in $rawArray) {
    # Env
    $osArchArr = $osArch.Split("/")
    $env:GOOS = $osArchArr[0]
    $env:GOARCH = $osArchArr[1]
    # Compile
    $filename = "cyrkensia-$env:GOOS-$env:GOARCH"
    if ($env:GOOS -eq "windows") {
        $filename += ".exe"
    }
    Set-Location src
    (go build -o ../build/$filename -ldflags="-s -w" cyrkensia.go) | Out-Null
    # Release
    if (Test-Path ../build/$filename -PathType Leaf) {
        $hash = (Get-FileHash -Algorithm SHA256 -LiteralPath "../build/$filename").Hash.ToLower()
        $releaseText += "* ``$filename``: $hash`n"
    }
    Set-Location ..
}
Clear-Host
Write-Output $releaseText
