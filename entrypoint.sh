#!/bin/bash -l
set -euo pipefail

if [ "${INPUT_NO-CHANGELOG}" != "false" ]
then
    echo "do not generate changelog. '${INPUT_NO-CHANGELOG}'"
    NO_CHANGELOG="--no-changelog"
else
    NO_CHANGELOG=""
fi

if [ ! -z "${INPUT_REGISTRY}" ]
then
    echo "using registry '${INPUT_REGISTRY}'"
    ALT_REGISTRY="--registry ${INPUT_REGISTRY}"
else
    ALT_REGISTRY=""
fi

release-plz release-pr\
    --github-token ${GITHUB_TOKEN}\
    --repo-url https://github.com/${GITHUB_REPOSITORY}\
    ${NO_CHANGELOG}\
    ${ALT_REGISTRY}\
    ${INPUT_ARGS}


exit_code=$?

# Pass exit code to the next step
echo "::set-output name=exit_code::$exit_code"
