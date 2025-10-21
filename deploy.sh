#!/bin/bash
set -e  # Exit on any error

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Starting deployment process...${NC}\n"

# Check if we're on a clean working tree
if [[ -n $(git status -s) ]]; then
    echo -e "${RED}❌ Working directory not clean. Please commit or stash changes first.${NC}"
    exit 1
fi

# Get current branch
CURRENT_BRANCH=$(git branch --show-current)
echo -e "${BLUE}📍 Current branch: ${CURRENT_BRANCH}${NC}\n"

# Step 1: Build Tailwind CSS
echo -e "${BLUE}🎨 Building Tailwind CSS...${NC}"
npm run css:build
echo -e "${GREEN}✅ Tailwind CSS built${NC}\n"

# Step 2: Build with Trunk
echo -e "${BLUE}🦀 Building Rust/WASM with Trunk...${NC}"
trunk build --release --public-url /
echo -e "${GREEN}✅ Trunk build complete${NC}\n"

# Step 3: Commit source changes to main (if any)
echo -e "${BLUE}💾 Committing source changes to main...${NC}"
git checkout main
if [[ -n $(git status -s src/ Cargo.toml Cargo.lock package.json input.css tailwind.config.js) ]]; then
    git add src/ Cargo.toml Cargo.lock package.json package-lock.json input.css tailwind.config.js index.html
    git commit -m "Update source code - $(date '+%Y-%m-%d %H:%M:%S')" || echo "No source changes to commit"
    git push origin main
    echo -e "${GREEN}✅ Source code pushed to main${NC}\n"
else
    echo -e "${GREEN}✅ No source changes to commit${NC}\n"
fi

# Step 4: Deploy to gh-branch
echo -e "${BLUE}🌐 Deploying to gh-branch...${NC}"
git subtree push --prefix dist origin gh-branch
echo -e "${GREEN}✅ Deployed to gh-branch${NC}\n"

# Step 5: Return to original branch
if [[ "$CURRENT_BRANCH" != "main" ]]; then
    echo -e "${BLUE}🔄 Returning to ${CURRENT_BRANCH}...${NC}"
    git checkout "$CURRENT_BRANCH"
fi

echo -e "${GREEN}🎉 Deployment complete!${NC}"
echo -e "${BLUE}📦 Main branch: Source code updated${NC}"
echo -e "${BLUE}🌍 gh-branch: Built site deployed${NC}"
echo -e "\n${BLUE}Your site should be live at: https://$(git config --get remote.origin.url | sed 's/.*github.com[:/]\(.*\)\.git/\1/' | cut -d'/' -f1).github.io/${NC}"
