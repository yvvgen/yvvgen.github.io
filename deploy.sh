#!/bin/bash
set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}🚀 Starting deployment...${NC}\n"

# Step 1: Build
echo -e "${BLUE}🎨 Building Tailwind CSS...${NC}"
npm run css:build

echo -e "${BLUE}🦀 Building with Trunk...${NC}"
trunk build --release --public-url /
echo -e "${GREEN}✅ Build complete${NC}\n"

# Step 2: Save current branch
CURRENT_BRANCH=$(git branch --show-current)

# Step 3: Commit source to main
echo -e "${BLUE}💾 Updating main branch...${NC}"
git checkout main
git add src/ Cargo.toml Cargo.lock package.json package-lock.json input.css tailwind.config.mjs index.html .github/ 2>/dev/null || true

if git diff --staged --quiet; then
    echo -e "${YELLOW}No source changes to commit${NC}"
else
    git commit -m "Update source - $(date '+%Y-%m-%d %H:%M:%S')"
    git push origin main
    echo -e "${GREEN}✅ Pushed to main${NC}"
fi

# Step 4: Deploy to gh-branch
echo -e "${BLUE}🌐 Deploying to gh-branch...${NC}"

# Create temporary directory
TMP_DIR=$(mktemp -d)
cp -r dist/* "$TMP_DIR/"

# Switch to gh-branch
git checkout gh-branch || git checkout --orphan gh-branch

# Remove everything except .git
find . -maxdepth 1 ! -name '.git' ! -name '.' ! -name '..' -exec rm -rf {} +

# Copy built files from temp
cp -r "$TMP_DIR"/* .

# Clean up temp
rm -rf "$TMP_DIR"

# Commit and push
git add .
git commit -m "Deploy - $(date '+%Y-%m-%d %H:%M:%S')" || echo "No changes to deploy"
git push origin gh-branch --force

echo -e "${GREEN}✅ Deployed to gh-branch${NC}\n"

# Step 5: Return to original branch
git checkout "$CURRENT_BRANCH"

echo -e "${GREEN}🎉 Deployment complete!${NC}"
