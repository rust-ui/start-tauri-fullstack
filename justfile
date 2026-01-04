# Cross-platform justfile for start-tauri-fullstack
# Install just: cargo install just

set dotenv-load := true

# Default recipe - show available commands
default:
    @just --list

# ============================================================================
# SETUP
# ============================================================================

# Ignore local IP changes in tauri config files
setup:
    git update-index --assume-unchanged src-tauri/tauri.ios.conf.json src-tauri/tauri.android.conf.json
    @echo "✅ Git will now ignore local IP changes in tauri config files"

# ============================================================================
# DATABASE
# ============================================================================

# Reset database (drop, create, migrate)
reset_db:
    @echo "Terminating all connections to database: $POSTGRES_DATABASE"
    psql -h 127.0.0.1 -U "$POSTGRES_USER" -d postgres -c "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '$POSTGRES_DATABASE';"
    @echo "Dropping database: $POSTGRES_DATABASE"
    psql -h 127.0.0.1 -U "$POSTGRES_USER" -d postgres -c "DROP DATABASE IF EXISTS $POSTGRES_DATABASE;"
    @echo "Creating database: $POSTGRES_DATABASE"
    psql -h 127.0.0.1 -U "$POSTGRES_USER" -d postgres -c "CREATE DATABASE $POSTGRES_DATABASE;"
    @echo "Running migrations with sqlx"
    sqlx migrate run
    @echo "Done!"

# ============================================================================
# DESKTOP
# ============================================================================

# Run Tauri desktop dev with custom port (reload_port = port + 1)
[macos]
run_desktop port="3000":
    #!/usr/bin/env bash
    set -e
    reload_port=$(({{port}} + 1))
    sed -i '' "s|http://localhost:[0-9]*|http://localhost:{{port}}|g" src-tauri/tauri.conf.json
    echo "Running on port {{port}} (reload: $reload_port)"
    LEPTOS_SITE_ADDR="127.0.0.1:{{port}}" LEPTOS_RELOAD_PORT="$reload_port" cargo tauri dev

[linux]
run_desktop port="3000":
    #!/usr/bin/env bash
    set -e
    reload_port=$(({{port}} + 1))
    sed -i "s|http://localhost:[0-9]*|http://localhost:{{port}}|g" src-tauri/tauri.conf.json
    echo "Running on port {{port}} (reload: $reload_port)"
    LEPTOS_SITE_ADDR="127.0.0.1:{{port}}" LEPTOS_RELOAD_PORT="$reload_port" cargo tauri dev

[windows]
run_desktop port="3000":
    #!powershell
    $reload_port = [int]{{port}} + 1
    (Get-Content src-tauri/tauri.conf.json) -replace 'http://localhost:[0-9]+', "http://localhost:{{port}}" | Set-Content src-tauri/tauri.conf.json
    Write-Host "Running on port {{port}} (reload: $reload_port)"
    $env:LEPTOS_SITE_ADDR = "127.0.0.1:{{port}}"
    $env:LEPTOS_RELOAD_PORT = "$reload_port"
    cargo tauri dev

# ============================================================================
# MOBILE - iOS (macOS only)
# ============================================================================

# Run iOS simulator (macOS only)
[macos]
run_ios port="3000" device="iPhone 16 Pro":
    #!/usr/bin/env bash
    set -e
    reload_port=$(({{port}} + 1))

    # Initialize iOS project if not already done
    if [ ! -d "src-tauri/gen/apple" ]; then
        echo "iOS project not initialized. Running cargo tauri ios init..."
        cargo tauri ios init
    fi

    SERVER_IP=$(ipconfig getifaddr en0)
    sed -i '' "s|http://[0-9.]*:[0-9]*|http://${SERVER_IP}:{{port}}|g" src-tauri/tauri.ios.conf.json
    echo "Updated SERVER_URL to http://${SERVER_IP}:{{port}}"

    APP_NAME=$(grep '^name' src-tauri/Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
    cp __HideKeyboardAccessory.m "src-tauri/gen/apple/Sources/${APP_NAME}/"
    cp __DisableContentInsetAdjustment.m "src-tauri/gen/apple/Sources/${APP_NAME}/"
    cd src-tauri/gen/apple && xcodegen generate && cd ../../..

    xcrun simctl boot "{{device}}" || true
    open -a Simulator
    echo "Running on port {{port}} (reload: $reload_port)"
    LEPTOS_SITE_ADDR="0.0.0.0:{{port}}" LEPTOS_RELOAD_PORT="$reload_port" cargo tauri ios dev "{{device}}"

[linux]
[windows]
run_ios port="" device="":
    @echo "Error: iOS development requires macOS"
    @exit 1

# ============================================================================
# MOBILE - Android (all platforms)
# ============================================================================

# Run Android emulator/device
[macos]
run_android port="3000":
    #!/usr/bin/env bash
    set -e
    reload_port=$(({{port}} + 1))
    SERVER_IP=$(ipconfig getifaddr en0)
    sed -i '' "s|http://[0-9.]*:[0-9]*|http://${SERVER_IP}:{{port}}|g" src-tauri/tauri.android.conf.json
    echo "Updated SERVER_URL to http://${SERVER_IP}:{{port}}"
    echo "Running on port {{port}} (reload: $reload_port)"
    LEPTOS_SITE_ADDR="0.0.0.0:{{port}}" LEPTOS_RELOAD_PORT="$reload_port" cargo tauri android dev

[linux]
run_android port="3000":
    #!/usr/bin/env bash
    set -e
    reload_port=$(({{port}} + 1))
    SERVER_IP=$(hostname -I | awk '{print $1}')
    sed -i "s|http://[0-9.]*:[0-9]*|http://${SERVER_IP}:{{port}}|g" src-tauri/tauri.android.conf.json
    echo "Updated SERVER_URL to http://${SERVER_IP}:{{port}}"
    echo "Running on port {{port}} (reload: $reload_port)"
    LEPTOS_SITE_ADDR="0.0.0.0:{{port}}" LEPTOS_RELOAD_PORT="$reload_port" cargo tauri android dev

[windows]
run_android port="3000":
    #!powershell
    $reload_port = [int]{{port}} + 1
    $SERVER_IP = (Get-NetIPAddress -AddressFamily IPv4 | Where-Object { $_.InterfaceAlias -notmatch 'Loopback' -and $_.PrefixOrigin -eq 'Dhcp' } | Select-Object -First 1).IPAddress
    (Get-Content src-tauri/tauri.android.conf.json) -replace 'http://[0-9.]+:[0-9]+', "http://${SERVER_IP}:{{port}}" | Set-Content src-tauri/tauri.android.conf.json
    Write-Host "Updated SERVER_URL to http://${SERVER_IP}:{{port}}"
    Write-Host "Running on port {{port}} (reload: $reload_port)"
    $env:LEPTOS_SITE_ADDR = "0.0.0.0:{{port}}"
    $env:LEPTOS_RELOAD_PORT = "$reload_port"
    cargo tauri android dev
