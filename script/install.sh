#!/usr/bin/env sh
set -eu

# Downloads a tarball from https://zed.dev/releases and unpacks it
# into ~/.local/. If you'd prefer to do this manually, instructions are at
# https://zed.dev/docs/linux.

main() {
    platform="$(uname -s)"
    arch="$(uname -m)"
    channel="${ZERMINAL_CHANNEL:-stable}"
    ZERMINAL_VERSION="${ZERMINAL_VERSION:-latest}"
    # Use TMPDIR if available (for environments with non-standard temp directories)
    if [ -n "${TMPDIR:-}" ] && [ -d "${TMPDIR}" ]; then
        temp="$(mktemp -d "$TMPDIR/zerminal-XXXXXX")"
    else
        temp="$(mktemp -d "/tmp/zerminal-XXXXXX")"
    fi

    if [ "$platform" = "Darwin" ]; then
        platform="macos"
    elif [ "$platform" = "Linux" ]; then
        platform="linux"
    else
        echo "Unsupported platform $platform"
        exit 1
    fi

    case "$platform-$arch" in
        macos-arm64* | linux-arm64* | linux-armhf | linux-aarch64)
            arch="aarch64"
            ;;
        macos-x86* | linux-x86* | linux-i686*)
            arch="x86_64"
            ;;
        *)
            echo "Unsupported platform or architecture"
            exit 1
            ;;
    esac

    if command -v curl >/dev/null 2>&1; then
        curl () {
            command curl -fL "$@"
        }
    elif command -v wget >/dev/null 2>&1; then
        curl () {
            wget -O- "$@"
        }
    else
        echo "Could not find 'curl' or 'wget' in your path"
        exit 1
    fi

    "$platform" "$@"

    if [ "$(command -v zed)" = "$HOME/.local/bin/zerminal" ]; then
        echo "Zerminal has been installed. Run with 'zerminal'"
    else
        echo "To run Zed from your terminal, you must add ~/.local/bin to your PATH"
        echo "Run:"

        case "$SHELL" in
            *zsh)
                echo "   echo 'export PATH=\$HOME/.local/bin:\$PATH' >> ~/.zshrc"
                echo "   source ~/.zshrc"
                ;;
            *fish)
                echo "   fish_add_path -U $HOME/.local/bin"
                ;;
            *)
                echo "   echo 'export PATH=\$HOME/.local/bin:\$PATH' >> ~/.bashrc"
                echo "   source ~/.bashrc"
                ;;
        esac

        echo "To run Zed now, '~/.local/bin/zerminal'"
    fi
}

linux() {
    if [ -n "${ZERMINAL_BUNDLE_PATH:-}" ]; then
        cp "$ZERMINAL_BUNDLE_PATH" "$temp/zerminal-linux-$arch.tar.gz"
    else
        echo "Downloading Zed version: $ZERMINAL_VERSION"
        curl "https://cloud.zed.dev/releases/$channel/$ZERMINAL_VERSION/download?asset=zerminal&arch=$arch&os=linux&source=install.sh" > "$temp/zerminal-linux-$arch.tar.gz"
    fi

    suffix=""
    if [ "$channel" != "stable" ]; then
        suffix="-$channel"
    fi

    appid=""
    case "$channel" in
      stable)
        appid="dev.zerminal.Zerminal"
        ;;
      nightly)
        appid="dev.zerminal.Zerminal-Nightly"
        ;;
      preview)
        appid="dev.zerminal.Zerminal-Preview"
        ;;
      dev)
        appid="dev.zerminal.Zerminal-Dev"
        ;;
      *)
        echo "Unknown release channel: ${channel}. Using stable app ID."
        appid="dev.zerminal.Zerminal"
        ;;
    esac

    # Unpack
    rm -rf "$HOME/.local/zerminal$suffix.app"
    mkdir -p "$HOME/.local/zerminal$suffix.app"
    tar -xzf "$temp/zerminal-linux-$arch.tar.gz" -C "$HOME/.local/"

    # Setup ~/.local directories
    mkdir -p "$HOME/.local/bin" "$HOME/.local/share/applications"

    # Link the binary
    if [ -f "$HOME/.local/zerminal$suffix.app/bin/zerminal" ]; then
        ln -sf "$HOME/.local/zerminal$suffix.app/bin/zerminal" "$HOME/.local/bin/zerminal"
    else
        # support for versions before 0.139.x.
        ln -sf "$HOME/.local/zerminal$suffix.app/bin/cli" "$HOME/.local/bin/zerminal"
    fi

    # Copy .desktop file
    desktop_file_path="$HOME/.local/share/applications/${appid}.desktop"
    src_dir="$HOME/.local/zed$suffix.app/share/applications"
    if [ -f "$src_dir/${appid}.desktop" ]; then
        cp "$src_dir/${appid}.desktop" "${desktop_file_path}"
    else
        # Fallback for older tarballs
        cp "$src_dir/zerminal$suffix.desktop" "${desktop_file_path}"
    fi
    sed -i "s|Icon=zerminal|Icon=$HOME/.local/zerminal$suffix.app/share/icons/hicolor/512x512/apps/zerminal.png|g" "${desktop_file_path}"
    sed -i "s|Exec=zerminal|Exec=$HOME/.local/zerminal$suffix.app/bin/zerminal|g" "${desktop_file_path}"
}

macos() {
    echo "Downloading Zed version: $ZERMINAL_VERSION"
    curl "https://cloud.zed.dev/releases/$channel/$ZERMINAL_VERSION/download?asset=zerminal&os=macos&arch=$arch&source=install.sh" > "$temp/Zerminal-$arch.dmg"
    hdiutil attach -quiet "$temp/Zerminal-$arch.dmg" -mountpoint "$temp/mount"
    app="$(cd "$temp/mount/"; echo *.app)"
    echo "Installing $app"
    if [ -d "/Applications/$app" ]; then
        echo "Removing existing $app"
        rm -rf "/Applications/$app"
    fi
    ditto "$temp/mount/$app" "/Applications/$app"
    hdiutil detach -quiet "$temp/mount"

    mkdir -p "$HOME/.local/bin"
    # Link the binary
    ln -sf "/Applications/$app/Contents/MacOS/cli" "$HOME/.local/bin/zerminal"
}

main "$@"
