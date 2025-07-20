#!/bin/bash

# ラベルを作成するスクリプト
# 使用方法: ./scripts/setup-labels.sh

echo "Setting up GitHub labels..."

# backend
gh label create "backend" --description "Backend related changes" --color "0052CC" --force

# frontend  
gh label create "frontend" --description "Frontend related changes" --color "FBCA04" --force

# documentation
gh label create "documentation" --description "Documentation changes" --color "0E8A16" --force

# ci/cd
gh label create "ci/cd" --description "CI/CD configuration changes" --color "1D76DB" --force

# dependencies
gh label create "dependencies" --description "Dependency updates" --color "EE3F46" --force

echo "✅ Labels created successfully!"