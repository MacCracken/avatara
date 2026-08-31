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

# Propagate the cyrius.cyml `[package].cyrius` toolchain pin into the three
# human-facing references that quote it by hand. The pin is bumped separately
# from the package version; this keeps the prose from going stale either way.
CYRIUS_PIN="$(grep '^cyrius = ' cyrius.cyml | head -1 | sed 's/cyrius = "\(.*\)"/\1/')"
sed -i "s/^- \*\*Compiler\*\*: cyrius >= [0-9][0-9.]*/- **Compiler**: cyrius >= ${CYRIUS_PIN}/" CLAUDE.md
sed -i "s/Cyrius compiler [0-9][0-9.]*+/Cyrius compiler ${CYRIUS_PIN}+/g" README.md

# Prepend a CHANGELOG stub under [Unreleased] if this version isn't there yet.
if ! grep -q "## \[${NEW_VERSION}\]" CHANGELOG.md; then
    awk -v v="$NEW_VERSION" -v d="$DATE" '
        /^## \[Unreleased\]/ { print; print ""; print "## [" v "] — " d; print ""; print "### Changed"; print "- "; next }
        { print }
    ' CHANGELOG.md > CHANGELOG.md.tmp && mv CHANGELOG.md.tmp CHANGELOG.md
fi

# Regenerate the consumer bundle so dist/avatara.cyr carries the new version.
command -v cyrius >/dev/null && cyrius distlib >/dev/null 2>&1 || true

echo "Bumped to ${NEW_VERSION} (cyrius pin ${CYRIUS_PIN})."
echo "Still manual: fill in CHANGELOG.md ${NEW_VERSION} section, then tag and push."
