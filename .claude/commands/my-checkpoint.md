Complete checkpoint: validate, commit, deploy, and push all changes.

This command ensures CHANGELOG, README.md, screenshot, and live demo are all up-to-date.

## Steps to Execute

### 1. Check for App Changes
Run `git status` and `git diff --name-only` to determine what changed.
- If ONLY documentation files changed (*.md, no .rs, .css, .html, .toml changes), skip screenshot step
- If ANY app code changed (.rs, .css, .html, .toml), screenshot is REQUIRED

### 2. CHANGELOG Generation
Run `./scripts/generate-changelog.sh "description of pending change"` to regenerate CHANGELOG from git history.
- Pass pending change description as argument (will show as `<latest>`)
- Or run without argument if no pending changes

### 3. Pre-commit Checks
Run `./scripts/pre-commit-checks.sh` and fix any issues.

### 4. Screenshot Update (REQUIRED for app code changes)

**This step is MANDATORY when any .rs, .css, .html, or .toml files have changed.**

Use MCP Playwright tools to capture a new screenshot:

```
Step 4a: Kill any existing server and start fresh
- lsof -ti:1140 | xargs kill -9 2>/dev/null
- ./scripts/serve.sh (run in background)
- Wait 5 seconds for server startup

Step 4b: Navigate with Playwright
- mcp__playwright__playwright_navigate to http://127.0.0.1:1140

Step 4c: Select tt2 mode (highest user level)
- mcp__playwright__playwright_click selector: "select#user-level"
- mcp__playwright__playwright_select selector: "select#user-level" value: "tt2"

Step 4d: Open help panel and expand Bird/Nest section
- mcp__playwright__playwright_click selector: "button.help-button"
- Wait 500ms for panel to open
- mcp__playwright__playwright_click selector: ".accordion-header" (the Bird/Nest Messaging one)

Step 4e: Take screenshot
- Get epoch: date +%s (store as EPOCH variable)
- Delete old screenshots: rm images/screenshot-*.png
- mcp__playwright__playwright_screenshot with:
  - name: "screenshot-{EPOCH}"
  - savePng: true
  - downloadsDir: "{PROJECT_ROOT}/images"
  - width: 1280
  - height: 800

Step 4f: Rename screenshot file
- mv images/screenshot-{EPOCH}*.png images/screenshot-{EPOCH}.png

Step 4g: Update README.md
- Edit README.md to update the screenshot link from old filename to new:
  ![tt-rs Screenshot](images/screenshot-{EPOCH}.png)

Step 4h: Close browser
- mcp__playwright__playwright_close
```

### 5. Commit Code Changes
If there are uncommitted changes:
```bash
git add -A
git commit -m "feat/fix/docs: <description>"
```

### 6. Build Release
```bash
./scripts/build-release.sh
```
Verify validation passes (PASS for all checks).

### 7. Deploy Commit
```bash
git add docs/
git commit -m "deploy: Update live demo with <description>"
```

### 8. Push
```bash
git push
```

### 9. Verification
After push, verify:
- Live demo works: https://sw-fun.github.io/tt-rs/
- README screenshot shows current UI (tt2 with Bird/Nest help expanded)
- CHANGELOG includes all recent commits with correct SHAs

## Important Notes

- Screenshot is MANDATORY for any app code changes - DO NOT SKIP
- Screenshot must show: tt2 mode selected, help panel open, Bird/Nest Messaging section expanded
- Use actual MCP Playwright tools, not manual instructions
- If any step fails, stop and fix before continuing
- CHANGELOG is auto-generated from git history via `./scripts/generate-changelog.sh`
