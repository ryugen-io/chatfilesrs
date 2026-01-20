#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="${SCRIPT_DIR}/../../.."
source "${SCRIPT_DIR}/../lib/log.sh"

SCOPE="BUILD"

cd "$PROJECT_ROOT"

log_info "$SCOPE" "=== starting build.sh ==="
log_info "$SCOPE" "working directory: $(pwd)"

# Parse arguments
RELEASE=false
QUIET=false
while [[ $# -gt 0 ]]; do
    case "$1" in
        --release|-r)
            RELEASE=true
            shift
            ;;
        --quiet|-q)
            QUIET=true
            shift
            ;;
        *)
            log_warn "$SCOPE" "unknown argument: $1"
            shift
            ;;
    esac
done

log_step "$SCOPE" "build configuration"
log_info "$SCOPE" "release mode: ${RELEASE}"
log_info "$SCOPE" "quiet mode: ${QUIET}"

# Build command
BUILD_CMD="cargo build --workspace"
if [[ "$RELEASE" == true ]]; then
    BUILD_CMD+=" --release"
fi
if [[ "$QUIET" == true ]]; then
    BUILD_CMD+=" --quiet"
fi

log_step "$SCOPE" "building workspace"
log_info "$SCOPE" "executing: ${BUILD_CMD}"

if eval "$BUILD_CMD"; then
    log_ok "$SCOPE" "build succeeded"
else
    log_error "$SCOPE" "build failed"
    exit 1
fi

# Show built artifacts
if [[ "$RELEASE" == true ]]; then
    TARGET_DIR="target/release"
else
    TARGET_DIR="target/debug"
fi

log_step "$SCOPE" "built artifacts"

# Find executables (excluding shared libs/dirs)
find "$TARGET_DIR" -maxdepth 1 -type f -executable | while read -r bin_path; do
    # Skip shared libraries if they are marked executable
    if [[ "$bin_path" == *.so ]] || [[ "$bin_path" == *.dylib ]]; then continue; fi
    
    name=$(basename "$bin_path")
    size=$(du -h "$bin_path" | cut -f1)
    log_ok "$SCOPE" "${name}: ${size}"
done

# Find static libraries
find "$TARGET_DIR" -maxdepth 1 -type f -name "*.rlib" | while read -r lib_path; do
    name=$(basename "$lib_path")
    size=$(du -h "$lib_path" | cut -f1)
    log_info "$SCOPE" "${name}: ${size}"
done

log_info "$SCOPE" "=== build.sh finished successfully ==="
