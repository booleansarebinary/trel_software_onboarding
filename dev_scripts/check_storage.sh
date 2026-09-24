#!/usr/bin/env bash
# Verifies that this checkout lives somewhere it is allowed to live.
#
# Run by setup.sh, and safe to run any time. It checks three things:
#   1. the repo is on local storage, not in a cloud-synced folder
#   2. full disk encryption is on
#   3. the Bazel cache (which contains copies of repo contents) is also local
#
# Read the "Where This Code Lives" section of the README for why. This script
# is a smoke test, not a compliance audit. Passing it does not make you
# compliant; failing it means you are definitely not.

set -uo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
FAILURES=0

fail() {
    printf '  [FAIL] %s\n' "$1"
    FAILURES=$((FAILURES + 1))
}

warn() { printf '  [WARN] %s\n' "$1"; }
pass() { printf '  [ ok ] %s\n' "$1"; }

echo "Checking where this checkout lives..."
echo "  path: $REPO_ROOT"

# --- 1. Cloud-synced directories -------------------------------------------
# Matches the sync roots people actually get bitten by. `pwd -P` above already
# resolved symlinks, so a symlink into iCloud will not sneak past this.
CLOUD_PATTERNS=(
    "Library/Mobile Documents"   # iCloud Drive, including Desktop & Documents sync
    "com~apple~CloudDocs"
    "Dropbox"
    "Google Drive"
    "GoogleDrive"
    "My Drive"
    "CloudStorage"               # macOS mount point for Drive/OneDrive/Box
    "OneDrive"
    "Box Sync"
    "Box"
    "pCloud"
    "Sync.com"
    "Nextcloud"
    "ownCloud"
    "Mega"
    "Creative Cloud Files"
)

cloud_hit=""
for pattern in "${CLOUD_PATTERNS[@]}"; do
    case "$REPO_ROOT" in
        *"$pattern"*)
            cloud_hit="$pattern"
            break
            ;;
    esac
done

if [ -n "$cloud_hit" ]; then
    fail "this checkout is inside a cloud-synced folder (matched \"$cloud_hit\")."
    echo "         Export-controlled source must not sync to a non-compliant cloud."
    echo "         Move it: mkdir -p ~/dev && mv \"$REPO_ROOT\" ~/dev/"
else
    pass "not in a recognized cloud-synced folder"
fi

# --- 2. Full disk encryption ----------------------------------------------
case "$(uname -s)" in
    Darwin)
        if command -v fdesetup >/dev/null 2>&1; then
            if fdesetup status 2>/dev/null | grep -q "FileVault is On"; then
                pass "FileVault is on"
            else
                fail "FileVault is OFF. Turn it on: System Settings > Privacy & Security > FileVault."
            fi
        else
            warn "could not run fdesetup; check FileVault manually"
        fi

        # macOS syncs ~/Desktop and ~/Documents to iCloud when "Desktop &
        # Documents Folders" is enabled, which silently uploads anything you
        # put there. The check above catches the synced path itself; this
        # catches a checkout sitting in the pre-migration location.
        case "$REPO_ROOT" in
            "$HOME/Desktop"/*|"$HOME/Documents"/*)
                warn "this is under ~/Desktop or ~/Documents. If iCloud Drive's"
                echo "         \"Desktop & Documents Folders\" is ever enabled, this gets"
                echo "         uploaded. Keep source in ~/dev instead."
                ;;
        esac
        ;;
    Linux)
        root_src="$(findmnt -n -o SOURCE --target "$REPO_ROOT" 2>/dev/null)"
        if [ -n "$root_src" ] && lsblk -no TYPE "$root_src" 2>/dev/null | grep -q crypt; then
            pass "filesystem holding this repo is on an encrypted device"
        elif command -v lsblk >/dev/null 2>&1 && lsblk -o TYPE 2>/dev/null | grep -q crypt; then
            warn "an encrypted volume exists but this repo may not be on it; verify with:"
            echo "         findmnt --target \"$REPO_ROOT\" && lsblk"
        else
            fail "no encrypted volume detected (expected LUKS/dm-crypt). Verify with lsblk."
        fi
        if grep -qi microsoft /proc/version 2>/dev/null; then
            warn "WSL detected: encryption is the WINDOWS host's job. Confirm BitLocker"
            echo "         is on for the drive backing this WSL distro."
        fi
        ;;
    *)
        warn "unrecognized OS; verify full disk encryption yourself"
        ;;
esac

# --- 3. Bazel cache location ---------------------------------------------
# The output base holds compiled copies of repo sources. It is as sensitive as
# the checkout and it is large enough that people relocate it and forget.
OUTPUT_BASE="$(bazel info output_base 2>/dev/null)"
if [ -n "$OUTPUT_BASE" ]; then
    cache_hit=""
    for pattern in "${CLOUD_PATTERNS[@]}"; do
        case "$OUTPUT_BASE" in
            *"$pattern"*)
                cache_hit="$pattern"
                break
                ;;
        esac
    done
    if [ -n "$cache_hit" ]; then
        fail "the Bazel output base is inside a cloud-synced folder (matched \"$cache_hit\")"
        echo "         output base: $OUTPUT_BASE"
    else
        pass "Bazel output base is local ($OUTPUT_BASE)"
    fi
else
    warn "could not determine the Bazel output base (is bazel installed?)"
fi

echo
if [ "$FAILURES" -gt 0 ]; then
    echo "$FAILURES check(s) failed. Worth sorting out before you write code -"
    echo "ask us if any of it is unclear."
    exit 1
fi
echo "Storage checks passed."
