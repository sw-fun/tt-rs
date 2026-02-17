#!/bin/bash
# Check if live demo is up to date with code changes.
# Run this before pushing to verify the live demo will be current.

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

echo "=== Checking Live Demo Status ==="

# Get the last commit that touched code files
LAST_CODE_COMMIT=$(git log -1 --format="%H" -- "components/" "*.rs" "*.toml" 2>/dev/null || echo "none")

# Get the last commit that touched docs/ (the deployed live demo)
LAST_DOCS_COMMIT=$(git log -1 --format="%H" -- "docs/" 2>/dev/null || echo "none")

# Check if docs/ has uncommitted changes
DOCS_DIRTY=$(git status --porcelain docs/ 2>/dev/null)

# Get commit timestamps
if [ "$LAST_CODE_COMMIT" != "none" ]; then
    CODE_TIME=$(git log -1 --format="%ct" "$LAST_CODE_COMMIT")
    CODE_MSG=$(git log -1 --format="%s" "$LAST_CODE_COMMIT")
else
    CODE_TIME=0
    CODE_MSG="(no code commits)"
fi

if [ "$LAST_DOCS_COMMIT" != "none" ]; then
    DOCS_TIME=$(git log -1 --format="%ct" "$LAST_DOCS_COMMIT")
    DOCS_MSG=$(git log -1 --format="%s" "$LAST_DOCS_COMMIT")
else
    DOCS_TIME=0
    DOCS_MSG="(no docs commits)"
fi

echo "Last code change: $CODE_MSG"
echo "Last docs deploy: $DOCS_MSG"

# Check if live demo is outdated
if [ "$CODE_TIME" -gt "$DOCS_TIME" ]; then
    echo ""
    echo "WARNING: Live demo is OUTDATED!"
    echo ""
    echo "Code was changed after the last deployment."
    echo ""
    echo "=== CLAUDE INSTRUCTIONS ==="
    echo "The live demo needs to be updated. Run:"
    echo ""
    echo "  ./scripts/build-release.sh"
    echo "  git add docs/"
    echo "  git commit -m 'deploy: Update live demo with <description>'"
    echo "  git push"
    echo ""
    echo "Then verify at https://sw-fun.github.io/tt-rs/"
    echo "==========================="
    exit 1
elif [ -n "$DOCS_DIRTY" ]; then
    echo ""
    echo "INFO: docs/ has uncommitted changes (build ready to commit)"
    echo ""
    echo "=== CLAUDE INSTRUCTIONS ==="
    echo "The live demo has been built but not committed. Run:"
    echo ""
    echo "  git add docs/"
    echo "  git commit -m 'deploy: Update live demo with <description>'"
    echo "  git push"
    echo ""
    echo "==========================="
    exit 0
else
    echo ""
    echo "PASS: Live demo is up to date!"
fi
