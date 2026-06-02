#!/usr/bin/env bash
# Atomically bump the project version across all files that track it.
# VERSION is the single source of truth; cyrius.cyml derives from it via
# the ${file:VERSION} template, so the manifest needs no edit here.
set -euo pipefail
[ $# -ne 1 ] && echo "Usage: $0 <version>" && exit 1
NEW_VERSION="$1"
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"
DATE="$(date -u +%Y-%m-%d)"

echo "$NEW_VERSION" > VERSION
sed -i "s/^- \*\*Version\*\*: .*/- **Version**: SemVer ${NEW_VERSION}/" CLAUDE.md

# Prepend a CHANGELOG stub under [Unreleased] if this version isn't there yet.
if ! grep -q "## \[${NEW_VERSION}\]" CHANGELOG.md; then
    awk -v v="$NEW_VERSION" -v d="$DATE" '
        /^## \[Unreleased\]/ { print; print ""; print "## [" v "] — " d; print ""; print "### Changed"; print "- "; next }
        { print }
    ' CHANGELOG.md > CHANGELOG.md.tmp && mv CHANGELOG.md.tmp CHANGELOG.md
fi

# Regenerate the consumer bundle so dist/avatara.cyr carries the new version.
command -v cyrius >/dev/null && cyrius distlib >/dev/null 2>&1 || true

echo "Bumped to ${NEW_VERSION}."
echo "Still manual: fill in CHANGELOG.md ${NEW_VERSION} section, then tag and push."
