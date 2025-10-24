#!/bin/bash
set -e

# Catppuccin Mocha Colors (24-bit)
GREEN='\033[38;2;171;233;179m'   # pastel green
BLUE='\033[38;2;137;180;250m'    # pastel blue
RED='\033[38;2;243;139;168m'     # pastel red
YELLOW='\033[38;2;249;226;175m'  # pastel yellow
NC='\033[0m'                      # reset

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

