#!/bin/bash
# Script to remove leaked .env file from Git history
# ⚠️ WARNING: This rewrites Git history!

set -e

echo "🚨 CRITICAL: Removing leaked .env file from Git history"
echo "========================================================="
echo ""
echo "⚠️  BEFORE running this script:"
echo "   1. REVOKE/ROTATE all API keys that were in the .env file!"
echo "   2. Notify all collaborators (they'll need to re-clone)"
echo "   3. Make a backup: git clone --mirror <repo-url> repo-backup"
echo ""
read -p "Have you REVOKED the exposed API keys? (yes/no): " confirmed

if [ "$confirmed" != "yes" ]; then
    echo "❌ Aborting. Please revoke API keys first!"
    exit 1
fi

echo ""
echo "📝 Files to remove from history:"
echo "   - glm-cc.sh (contains keys)"
echo "   - local_ai_assistant/.env.example (if it has real keys)"
echo ""

# Navigate to repo root
cd /Volumes/UltraDisk/Dev2/crypto-projects/iDoris

echo "🔧 Step 1: Removing glm-cc.sh from all history..."
git filter-branch --force --index-filter \
  "git rm --cached --ignore-unmatch glm-cc.sh" \
  --prune-empty --tag-name-filter cat -- --all

echo ""
echo "🔧 Step 2: Cleaning up..."
rm -rf .git/refs/original/
git reflog expire --expire=now --all
git gc --prune=now --aggressive

echo ""
echo "✅ Local history cleaned!"
echo ""
echo "🚀 Step 3: Force push to GitHub (this REWRITES remote history)..."
read -p "Ready to force push? This affects all collaborators! (yes/no): " push_confirmed

if [ "$push_confirmed" == "yes" ]; then
    git push origin --force --all
    git push origin --force --tags
    echo ""
    echo "✅ Done! The file has been removed from GitHub history."
    echo ""
    echo "📢 IMPORTANT NEXT STEPS:"
    echo "   1. All collaborators must run: git fetch origin && git reset --hard origin/main"
    echo "   2. Or better: re-clone the repository"
    echo "   3. Check GitHub commit page - may take a few minutes to update"
    echo "   4. Contact GitHub support if the commit is still visible after 24h"
else
    echo "❌ Force push skipped. Run this when ready:"
    echo "   git push origin --force --all"
    echo "   git push origin --force --tags"
fi

echo ""
echo "⚠️  REMEMBER: Git history rewrite doesn't revoke compromised keys!"
echo "   You MUST rotate/revoke all exposed credentials immediately!"
