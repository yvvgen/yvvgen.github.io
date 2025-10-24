#!/bin/bash
set -e

# Catppuccin Mocha Colors (24-bit)
GREEN='\033[38;2;171;233;179m'
BLUE='\033[38;2;137;180;250m'
RED='\033[38;2;243;139;168m'
YELLOW='\033[38;2;249;226;175m'
NC='\033[0m'

REPO="git@github.com:yvvgen/yvvgen.github.io.git"
GH_BRANCH="gh-branch"

echo -e "${BLUE}🚀 Starting deployment...${NC}\n"

# Ensure on dev branch
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [ "$CURRENT_BRANCH" != "dev" ]; then
    echo -e "${RED}❌ Not on dev branch: $CURRENT_BRANCH${NC}"
    echo -e "${BLUE}Switch with: git checkout dev${NC}"
    exit 1
fi

# Commit uncommitted changes
if ! git diff-index --quiet HEAD --; then
    echo -e "${YELLOW}⚠️  Uncommitted changes detected. Committing...${NC}"
    git add .
    git commit -m "Update: $(date '+%Y-%m-%d %H:%M:%S')"
fi

# Push dev branch
echo -e "${BLUE}⬆️  Pushing dev...${NC}"
git push origin dev
echo -e "${GREEN}✅ Dev pushed${NC}\n"

# Pull latest main and rebase dev
git checkout main
git pull origin main
git checkout dev
git rebase main
git push origin dev --force
echo -e "${GREEN}✅ Dev rebased onto main${NC}\n"

# Build the project
echo -e "${BLUE}🦀 Building with Trunk...${NC}"
trunk build --release --public-url /

# Check build output
if [ ! -d "dist" ] || [ -z "$(ls -A dist)" ]; then
    echo -e "${RED}❌ dist folder is empty or missing${NC}"
    exit 1
fi

if [ ! -f "dist/index.html" ]; then
    echo -e "${RED}❌ No index.html in dist/${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Build complete${NC}\n"

# Deploy from temporary folder
TMP_DIR=$(mktemp -d)
echo -e "${BLUE}📦 Preparing temporary deployment folder...${NC}"

# Clone gh-branch only into temp folder
git clone --branch $GH_BRANCH --single-branch $REPO "$TMP_DIR" || {
    echo -e "${YELLOW}⚠️  gh-branch does not exist yet, creating new branch...${NC}"
    git clone $REPO "$TMP_DIR"
    cd "$TMP_DIR"
    git checkout --orphan $GH_BRANCH
    git rm -rf . >/dev/null 2>&1 || true
    cd -
}

# Copy built files
cp -r dist/* "$TMP_DIR/"

# Commit and push
cd "$TMP_DIR"
git add .
if git commit -m "Deploy $(date '+%Y-%m-%d %H:%M:%S')"; then
    echo -e "${BLUE}⬆️  Pushing to $GH_BRANCH...${NC}"
    git push origin $GH_BRANCH --force
    echo -e "${GREEN}✅ Deployed to $GH_BRANCH${NC}\n"
else
    echo -e "${BLUE}No changes to deploy${NC}\n"
fi

# Return to original repo
cd -
echo -e "${BLUE}📍 Back to dev branch${NC}"
git checkout dev

echo -e "${GREEN}🎉 Deployment complete!${NC}"
echo -e "${BLUE}📍 Site: https://yvvgen.github.io${NC}"

