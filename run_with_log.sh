#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -lt 2 ]; then
    echo "Usage: $0 LOG_FILE COMMAND [ARGS...]" >&2
    exit 1
fi

log_file=$1
shift

cmd=("$@")

printf '[%s] Running: %s\n' "$(date)" "${cmd[*]}" | tee -a "$log_file"

set +e
"${cmd[@]}" 2>&1 | tee -a "$log_file"
status=${PIPESTATUS[0]}
set -e

printf '[%s] Exit status: %s\n' "$(date)" "$status" | tee -a "$log_file"

exit "$status"
