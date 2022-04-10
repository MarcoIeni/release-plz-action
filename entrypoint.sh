#!/bin/bash -l
set -euo pipefail

if [ "${INPUT_NO_CHANGELOG}" != "false" ]
then
    echo "do not generate changelog. '${INPUT_NO_CHANGELOG}'"
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

export PATH="/usr/local/cargo/bin:$PATH"

git config --global user.email "release-plz@github.com"
git config --global user.name "release-plz"

release-plz release-pr\
    --github-token ${GITHUB_TOKEN}\
    --repo-url https://github.com/${GITHUB_REPOSITORY}\
    ${NO_CHANGELOG}\
    ${ALT_REGISTRY}\
    ${INPUT_ARGS}


exit_code=$?

# Pass exit code to the next step
echo "::set-output name=exit_code::$exit_code"
