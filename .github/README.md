# GitHub Actions Setup

This directory contains the GitHub Actions workflows for CI/CD automation.

## Required Secrets

For the release workflow to work properly, you need to set up a Personal Access Token (PAT) in your repository secrets:

1. Create a new Personal Access Token (PAT) with the `repo` scope:
   - Go to GitHub Settings > Developer settings > Personal access tokens > Tokens (classic)
   - Click "Generate new token (classic)"
   - Give it a name like "Release Workflow"
   - Select the `repo` scope
   - Generate the token and copy it

2. Add the token as a secret in your repository:
   - Go to your repository Settings > Secrets and variables > Actions
   - Click "New repository secret"
   - Name it `GITHUB_PAT`
   - Paste the token value
   - Click "Add secret"

The release workflow uses `GITHUB_PAT` instead of the default `GITHUB_TOKEN` because the default token has limited permissions when workflows are triggered by tag pushes.