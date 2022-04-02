#!/bin/bash -l
set -euo pipefail

release-plz $@
exit_code=$?

# Pass exit code to the next step
echo "::set-output name=exit_code::$exit_code"
