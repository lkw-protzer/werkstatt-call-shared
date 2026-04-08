#!/usr/bin/env bash
# Validate schema/openapi.yaml against the OpenAPI 3.1 specification.
# Requires Node.js (npx will download @redocly/cli on first run).
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
YAML="${SCRIPT_DIR}/../schema/openapi.yaml"

echo "Validating: ${YAML}"
npx --yes @redocly/cli lint "${YAML}"
echo "OK: schema/openapi.yaml is valid OpenAPI 3.1"
