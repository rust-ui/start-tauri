# Cross-platform justfile for start-tauri
# Install just: cargo install just

# Default recipe - show available commands
default:
    @just --list

# ============================================================================
# MOBILE - iOS (macOS only)
# ============================================================================

# Run iOS simulator (macOS only)
[macos]
run_ios device="iPhone 16 Pro":
    #!/usr/bin/env bash
    set -e
    SERVER_IP=$(ipconfig getifaddr en0)
    sed -i '' "s|http://[0-9.]*:3000|http://${SERVER_IP}:3000|g" src-tauri/tauri.ios.conf.json
    echo "Updated SERVER_URL to http://${SERVER_IP}:3000"

    APP_NAME=$(grep '^name' src-tauri/Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
    cp __HideKeyboardAccessory.m "src-tauri/gen/apple/Sources/${APP_NAME}/"
    cp __DisableContentInsetAdjustment.m "src-tauri/gen/apple/Sources/${APP_NAME}/"
    cd src-tauri/gen/apple && xcodegen generate && cd ../../..

    xcrun simctl boot "{{device}}" || true
    open -a Simulator
    LEPTOS_SITE_ADDR="0.0.0.0:3000" cargo tauri ios dev "{{device}}"

[linux]
[windows]
run_ios device="":
    @echo "Error: iOS development requires macOS"
    @exit 1

# ============================================================================
# MOBILE - Android (all platforms)
# ============================================================================

# Run Android emulator/device
[macos]
run_android:
    #!/usr/bin/env bash
    set -e
    SERVER_IP=$(ipconfig getifaddr en0)
    sed -i '' "s|http://[0-9.]*:3000|http://${SERVER_IP}:3000|g" src-tauri/tauri.android.conf.json
    echo "Updated SERVER_URL to http://${SERVER_IP}:3000"
    LEPTOS_SITE_ADDR="0.0.0.0:3000" cargo tauri android dev

[linux]
run_android:
    #!/usr/bin/env bash
    set -e
    SERVER_IP=$(hostname -I | awk '{print $1}')
    sed -i "s|http://[0-9.]*:3000|http://${SERVER_IP}:3000|g" src-tauri/tauri.android.conf.json
    echo "Updated SERVER_URL to http://${SERVER_IP}:3000"
    LEPTOS_SITE_ADDR="0.0.0.0:3000" cargo tauri android dev

[windows]
run_android:
    #!powershell
    $SERVER_IP = (Get-NetIPAddress -AddressFamily IPv4 | Where-Object { $_.InterfaceAlias -notmatch 'Loopback' -and $_.PrefixOrigin -eq 'Dhcp' } | Select-Object -First 1).IPAddress
    (Get-Content src-tauri/tauri.android.conf.json) -replace 'http://[0-9.]+:3000', "http://${SERVER_IP}:3000" | Set-Content src-tauri/tauri.android.conf.json
    Write-Host "Updated SERVER_URL to http://${SERVER_IP}:3000"
    $env:LEPTOS_SITE_ADDR = "0.0.0.0:3000"
    cargo tauri android dev
