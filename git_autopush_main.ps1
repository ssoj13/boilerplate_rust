$commitMessage = Read-Host "Enter commit message"
Write-Host "Committing `"$commitMessage`""
git add -A .
if ($LASTEXITCODE -eq 0) {
    git commit -a -m "$commitMessage"
    if ($LASTEXITCODE -eq 0) {
        git push --recurse-submodules=on-demand --set-upstream origin main
    }
}