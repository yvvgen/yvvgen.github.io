#!/bin/bash
set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}🚀 Starting deployment...${NC}\n"

# Save current branch
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)

# Make sure we're on dev
if [ "$CURRENT_BRANCH" != "dev" ]; then
    echo -e "${RED}❌ Error: Not on dev branch. Currently on: $CURRENT_BRANCH${NC}"
    echo -e "${BLUE}Switch to dev with: git checkout dev${NC}"
    exit 1
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
    echo -e "${YELLOW}⚠️  You have uncommitted changes. Committing them first...${NC}"
    git add .
    git commit -m "Update: $(date '+%Y-%m-%d %H:%M:%S')"
fi

# Push dev branch
echo -e "${BLUE}⬆️  Pushing dev branch...${NC}"
git push origin dev
echo -e "${GREEN}✅ Pushed to dev${NC}\n"

# Switch to main and pull latest
echo -e "${BLUE}🔄 Switching to main...${NC}"
git checkout main
git pull origin main
echo -e "${GREEN}✅ Main is up to date${NC}\n"

# Rebase dev onto main
echo -e "${BLUE}🔀 Rebasing dev onto main...${NC}"
git rebase dev

# Push main
echo -e "${BLUE}⬆️  Pushing main...${NC}"
git push origin main
echo -e "${GREEN}✅ Pushed to main${NC}\n"

# Build with Trunk (includes Tailwind)
echo -e "${BLUE}🦀 Building with Trunk...${NC}"
trunk build --release --public-url /

echo -e "${GREEN}✅ Build complete${NC}\n"

# Check if dist directory exists and has content
if [ ! -d "dist" ] || [ -z "$(ls -A dist)" ]; then
    echo -e "${RED}❌ Error: dist directory is empty or doesn't exist${NC}"
    exit 1
fi

# Check for index.html
if [ ! -f "dist/index.html" ]; then
    echo -e "${RED}❌ ERROR: No index.html found in dist/${NC}"
    exit 1
fi

# Create temporary directory for built files
TMP_DIR=$(mktemp -d)
cp -r dist/* "$TMP_DIR/"

# Switch to gh-branch (or create if it doesn't exist)
echo -e "${BLUE}📦 Preparing gh-branch...${NC}"
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

# Clean up
rm -rf "$TMP_DIR"

# Commit and push to gh-branch
git add .
if git commit -m "Deploy $(date '+%Y-%m-%d %H:%M:%S')"; then
    echo -e "${BLUE}⬆️  Pushing to gh-branch...${NC}"
    git push origin gh-branch --force
    echo -e "${GREEN}✅ Deployed to gh-branch${NC}\n"
else
    echo -e "${BLUE}No changes to deploy${NC}\n"
fi

# Go back to dev branch
git checkout dev

# Update dev to match main (since we rebased)
echo -e "${BLUE}🔄 Updating dev to match main...${NC}"
git rebase main
git push origin dev --force

echo -e "${GREEN}🎉 Deployment complete!${NC}"
echo -e "${BLUE}📍 Site: https://yvvgen.github.io${NC}"
echo -e "${BLUE}📍 Back on dev branch${NC}\n"
