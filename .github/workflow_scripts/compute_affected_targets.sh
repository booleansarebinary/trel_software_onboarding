#!/usr/bin/env bash
# Narrows a list of Bazel targets down to the ones a set of changed files can
# actually affect.
#
# A simplified version of the script the main repo uses, which does the same
# thing with a shared remote cache attached.
#
# Usage:
#   compute_affected_targets <TARGETS> <CHANGED_FILES> <OUTPUT_VAR>
#     $1 = space-separated Bazel target labels
#     $2 = space-separated file paths
#     $3 = name of the GitHub Actions output to write
#
# The idea, and the reason Bazel is worth the trouble:
#
# `rdeps(set(TARGETS), set(FILES))` asks Bazel for every target in TARGETS that
# depends, directly or transitively, on any of those files. Bazel knows the
# exact dependency graph of the whole repo, so this is an answer rather than a
# guess. A hand-written CI script would have to approximate it with path globs,
# and would be wrong in both directions: it would run too much on most PRs and
# miss the one downstream target that actually broke.

compute_affected_targets() {
    if [ $# -ne 3 ]; then
        echo "Usage: compute_affected_targets <TARGETS> <CHANGED_FILES> <OUTPUT_VAR>"
        exit 1
    fi

    local TARGETS="$1"
    local CHANGED_FILES="$2"
    local OUTPUT_VAR="$3"

    local RDEPS
    local EXIT_CODE
    # --keep_going, and tolerating exit code 3, because a PR that deletes a
    # file leaves the query referring to something that no longer exists. That
    # is a partial-failure, not a broken pipeline.
    set +e
    RDEPS=$(bazel query "rdeps(set($TARGETS), set($CHANGED_FILES))" --keep_going 2>/dev/null)
    EXIT_CODE=$?
    set -e
    if [ $EXIT_CODE -ne 0 ] && [ $EXIT_CODE -ne 3 ]; then
        echo "bazel query failed with exit code $EXIT_CODE"
        exit 1
    fi

    # rdeps can return targets outside the kind we asked about, so intersect
    # back down to the original list.
    local AFFECTED
    AFFECTED=$(comm -12 \
        <(echo "$RDEPS" | sort -u) \
        <(echo "$TARGETS" | tr ' ' '\n' | sort -u) | tr '\n' ' ' | xargs)

    echo "$OUTPUT_VAR=$AFFECTED" >>"$GITHUB_OUTPUT"
}
