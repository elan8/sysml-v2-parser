#!/usr/bin/env bash
# Differential parse against OpenMBEE sysml-toolkit's parser (docs/PARSER-ORACLE.md), over two
# corpora: the pinned SysML v2 release, and the OpenSysML fixtures (mostly invalid models) the
# oracle repository vendors at the pinned revision.
#
#   scripts/parser-oracle.sh            # compare against the tests/parser_oracle_*.tsv baselines
#   scripts/parser-oracle.sh --update   # record the current divergences as the new baselines
#
# Reports go to conformance-out/parser-oracle-<corpus>.md. Needs the pinned release
# (scripts/fetch-sysml-v2-release.sh) and network access to clone the oracle once into .oracle/
# (sparse: its syntax crate and the fixture directory only; no submodules).
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
TARGET_FILE="${REPO_ROOT}/docs/parser-oracle-target"

read_target_value() {
  local value
  value="$(grep -E "^$1=" "${TARGET_FILE}" | head -n1 | cut -d= -f2- | tr -d '\r')"
  if [[ -z "${value}" ]]; then
    echo "Missing $1 in ${TARGET_FILE}" >&2
    exit 1
  fi
  printf '%s' "${value}"
}

ORACLE_REPO="$(read_target_value oracle_repo)"
ORACLE_REV="$(read_target_value oracle_rev)"
ORACLE_CRATE="$(read_target_value oracle_crate)"
ORACLE_FIXTURES="$(read_target_value oracle_fixtures)"
ORACLE_DIR="${REPO_ROOT}/.oracle/sysml-toolkit"
CORPUS="${SYSML_V2_RELEASE_DIR:-${REPO_ROOT}/sysml-v2-release}"
REPORT_DIR="${REPO_ROOT}/conformance-out"

if [[ ! -d "${CORPUS}" ]]; then
  echo "SysML v2 release not found at ${CORPUS}; run scripts/fetch-sysml-v2-release.sh" >&2
  exit 2
fi

if [[ "$(git -C "${ORACLE_DIR}" rev-parse HEAD 2>/dev/null || true)" != "${ORACLE_REV}" \
  || ! -d "${ORACLE_DIR}/${ORACLE_CRATE}" || ! -d "${ORACLE_DIR}/${ORACLE_FIXTURES}" ]]; then
  rm -rf "${ORACLE_DIR}"
  git init -q "${ORACLE_DIR}"
  git -C "${ORACLE_DIR}" remote add origin "${ORACLE_REPO}"
  git -C "${ORACLE_DIR}" sparse-checkout set "${ORACLE_CRATE}" "${ORACLE_FIXTURES}"
  git -C "${ORACLE_DIR}" fetch -q --depth 1 --filter=blob:none origin "${ORACLE_REV}"
  git -C "${ORACLE_DIR}" checkout -q FETCH_HEAD
fi

cargo build --quiet --release --manifest-path "${REPO_ROOT}/tools/parser_oracle/Cargo.toml"
ORACLE_BIN="${REPO_ROOT}/tools/parser_oracle/target/release/parser_oracle"
mkdir -p "${REPORT_DIR}"

status=0
run_corpus() {
  local name="$1" root="$2"
  shift 2
  echo "== ${name}: ${root}"
  "${ORACLE_BIN}" "${root}" \
    --baseline "${REPO_ROOT}/tests/parser_oracle_${name}.tsv" \
    --report "${REPORT_DIR}/parser-oracle-${name}.md" "$@" >/dev/null || status=$?
  grep -E '^\| [a-z-]+ \| [0-9]+ \|$' "${REPORT_DIR}/parser-oracle-${name}.md" || true
}
run_corpus release "${CORPUS}" "$@"
run_corpus opensysml "${ORACLE_DIR}/${ORACLE_FIXTURES}" "$@"
exit "${status}"
