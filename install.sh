#!/bin/sh
set -euf

ansi_red=""
ansi_green=""
ansi_blue=""
ansi_yellow=""
ansi_bold=""
ansi_reset=""

if [ -t 1 ] && command -v tput > /dev/null 2>&1; then
    ansi_red="$(tput setaf 1 || true)"
    ansi_green="$(tput setaf 2 || true)"
    ansi_yellow="$(tput setaf 3 || true)"
    ansi_blue="$(tput setaf 4 || true)"
    ansi_bold="$(tput bold || true)"
    ansi_reset="$(tput sgr0 || true)"
fi

success() {
    printf "${ansi_green}success${ansi_reset}  %s\n" "$*"
}

info() {
    printf "${ansi_blue}info${ansi_reset}     %s\n" "$*"
}

err() {
    printf "${ansi_red}error${ansi_reset}    %s\n" "$*"
}

warn() {
    printf "${ansi_yellow}warn${ansi_reset}     %s\n" "$*"
}

bold() {
    printf "${ansi_bold}%s${ansi_reset}" "$*"
}

main() {
    platform="$(uname -ms)"

    case $platform in
        "Darwin arm64") target="aarch64-apple-darwin" ;;
        "Darwin x86_64") target="x86_64-apple-darwin" ;;
        "Linux aarch64" | "Linux arm64") target="aarch64-unknown-linux-musl" ;;
        "Linux x86_64") target="x86_64-unknown-linux-musl" ;;
    esac

    if [ -z "$target" ]; then
        if command -v cargo > /dev/null 2>&1; then
            warn "GitHub Releases binary not found, falling back to cargo install"
            cargo install nrr
        else
            err "Unsupported platform! Could not find binary to download from GitHub Releases"
            exit 1
        fi
    fi

    info "Downloading binary for target $(bold "$target")..."

    asset_temp="$(mktemp)"
    curl -fSL "https://github.com/ryanccn/nrr/releases/latest/download/nrr-$target.zip" > "$asset_temp"

    unpack_temp="$(mktemp -d)"
    unzip "$asset_temp" -d "$unpack_temp" > /dev/null 2>&1

    cargo_home="${CARGO_HOME:-"$HOME/.cargo"}"
    mkdir -p "$cargo_home/bin"

    cp "$unpack_temp/nrr" "$cargo_home/bin"
    chmod +x "$cargo_home/bin/nrr"

    version="$(bold "$("$cargo_home/bin/nrr" --version)")"

    success "Installed $version to $cargo_home/bin ^^"

    case :$PATH:
        in *:"$cargo_home/bin":*) ;;
        *) warn "The installation directory is not in your PATH. You might want to add it in order to use nrr :p" >&2 ;;
    esac
}

main || exit 1
