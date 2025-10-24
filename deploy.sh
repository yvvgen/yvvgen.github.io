#!/bin/bash
set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}🚀 Starting deployment...${NC}\n"

# Save current branch
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)

# Build with Trunk (includes Tailwind)
echo -e "${BLUE}🦀 Building with Trunk...${NC}"
trunk build --release --public-url /

echo -e "${GREEN}✅ Build complete${NC}\n"

# Debug: Show what was built
echo -e "${BLUE}📦 Contents of dist/:${NC}"
ls -la dist/
echo ""

# Check for index.html
if [ ! -f "dist/index.html" ]; then
    echo -e "${RED}❌ ERROR: No index.html found in dist/${NC}"
    exit 1
fi

# Create temporary directory for built files
TMP_DIR=$(mktemp -d)
cp -r dist/* "$TMP_DIR/"

# Switch to gh-branch (or create if it doesn't exist)
if git show-ref --quiet refs/heads/gh-branch; then
    git checkout gh-branch
else
    git checkout --orphan gh-branch
    git rm -rf . >/dev/null 2>&1 || true
fi

# Remove everything except .git
find . -maxdepth 1 ! -name '.git' ! -name '.' ! -name '..' -exec rm -rf {} +

# Copy built files
cp -r "$TMP_DIR"/* .

# Debug: Show what's being deployed
echo -e "${BLUE}📦 Files to be deployed:${NC}"
ls -la
echo ""

# Clean up
rm -rf "$TMP_DIR"

# Commit and push (force to overwrite old site)
git add .
if git commit -m "Deploy $(date '+%Y-%m-%d %H:%M:%S')"; then
    git push origin gh-branch --force
    echo -e "${GREEN}🎉 Deployment complete!${NC}\n"
else
    echo -e "${BLUE}No changes to deploy${NC}\n"
fi

# Go back to original branch
git checkout "$CURRENT_BRANCH"

echo -e "${GREEN}✅ Returned to ${CURRENT_BRANCH}${NC}"
