#!/usr/bin/env bash
set -e

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
DRY_RUN=false
CONFIRM=true

# Parse arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --dry-run) DRY_RUN=true ;;
        -y|--yes) CONFIRM=false ;;
        *) echo "Unknown parameter passed: $1"; exit 1 ;;
    esac
    shift
done

echo -e "${YELLOW}Starting Release Process...${NC}"

# 1. Run Tests
echo "Running tests..."
if cargo test; then
    echo -e "${GREEN}Tests passed.${NC}"
    # Cleanup test artifacts
    git restore demo/assets_manifest.json 2>/dev/null || true
else
    echo -e "${RED}Tests failed. Aborting release.${NC}"
    exit 1
fi

# 2. Check Git Status
if [[ -n $(git status -s) ]]; then
    echo -e "${RED}Error: Uncommitted changes found. Please commit or stash them.${NC}"
    git status -s
    exit 1
fi

# 3. Get Version
# Extracts version from the first occurrence of version = "..." in Cargo.toml
VERSION=$(grep "^version =" Cargo.toml | head -n 1 | cut -d '"' -f 2)

if [[ -z "$VERSION" ]]; then
    echo -e "${RED}Error: Could not detect version from Cargo.toml${NC}"
    exit 1
fi

TAG="v$VERSION"
echo -e "Detected version: ${GREEN}$VERSION${NC}"
echo -e "Target Tag: ${GREEN}$TAG${NC}"

# 4. Check if tag exists
if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo -e "${RED}Error: Tag $TAG already exists.${NC}"
    exit 1
fi

# 5. Create Tag
if [ "$DRY_RUN" = true ]; then
    echo -e "${YELLOW}[DRY RUN] Would create tag: $TAG${NC}"
else
    if [ "$CONFIRM" = true ]; then
        read -p "Create tag $TAG? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "Aborted."
            exit 0
        fi
    fi

    git tag -a "$TAG" -m "Release $TAG"
    echo -e "${GREEN}Success! Tag $TAG created.${NC}"
    echo "To push this tag, run:"
    echo -e "${YELLOW}git push origin $TAG${NC}"
fi
