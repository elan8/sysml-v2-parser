#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-2026-01}"
DESTINATION="${2:-sysml-v2-release}"
REPO_NAME="SysML-v2-Release"
ARCHIVE_URL="https://github.com/Systems-Modeling/${REPO_NAME}/archive/refs/tags/${VERSION}.tar.gz"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [[ "$DESTINATION" = /* ]]; then
  DESTINATION_PATH="$DESTINATION"
else
  DESTINATION_PATH="$(cd "${SCRIPT_DIR}/.." && pwd)/${DESTINATION}"
fi

PARENT_DIR="$(dirname "$DESTINATION_PATH")"
TEMP_ROOT="$(mktemp -d)"
ARCHIVE_PATH="${TEMP_ROOT}/${VERSION}.tar.gz"
EXTRACT_DIR="${TEMP_ROOT}/extract"
EXPANDED_ROOT="${EXTRACT_DIR}/${REPO_NAME}-${VERSION}"
STAGING_PATH="${PARENT_DIR}/sysml-v2-release-stage-$(date +%s)-$$"

cleanup() {
  rm -rf "$STAGING_PATH" "$TEMP_ROOT"
}

trap cleanup EXIT

assert_release_layout() {
  local root="$1"
  local required=(
    "${root}/sysml"
    "${root}/sysml/src/validation"
    "${root}/sysml.library"
  )

  for path in "${required[@]}"; do
    if [[ ! -e "$path" ]]; then
      echo "Downloaded archive is missing expected path: $path" >&2
      exit 1
    fi
  done
}

mkdir -p "$PARENT_DIR" "$EXTRACT_DIR"

echo "Downloading ${REPO_NAME} ${VERSION} from ${ARCHIVE_URL}"
curl -L --fail --output "$ARCHIVE_PATH" "$ARCHIVE_URL"

echo "Extracting archive to temporary directory"
tar -xzf "$ARCHIVE_PATH" -C "$EXTRACT_DIR"

if [[ ! -d "$EXPANDED_ROOT" ]]; then
  echo "Expected extracted directory not found: $EXPANDED_ROOT" >&2
  exit 1
fi

assert_release_layout "$EXPANDED_ROOT"
mv "$EXPANDED_ROOT" "$STAGING_PATH"

if [[ -e "$DESTINATION_PATH" ]]; then
  echo "Removing existing destination ${DESTINATION_PATH}"
  rm -rf "$DESTINATION_PATH"
fi

mv "$STAGING_PATH" "$DESTINATION_PATH"
echo "SysML v2 release ${VERSION} is ready at ${DESTINATION_PATH}"
