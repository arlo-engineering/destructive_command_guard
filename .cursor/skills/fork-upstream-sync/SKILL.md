---
name: fork-upstream-sync
description: Sync this fork with its upstream repository using a fork-owned branch and GitHub Copilot MCP when available. Use when the user asks to sync with the parent repo, refresh a fork from upstream, resolve upstream merge conflicts, or repair a conflicted sync pull request.
---

# Fork Upstream Sync

Use this workflow when updating `arlo-engineering/destructive_command_guard` from its parent repository.

## Goal

Keep sync work repairable from the fork. The head branch for the sync PR must live in the fork, not in the upstream repository.

## Default Workflow

1. Confirm the remotes:
   - `origin` should be the fork
   - `upstream` should be the parent repository
2. Fetch both `origin/main` and `upstream/main`.
3. Create a fresh branch from `origin/main` named like `sync-upstream-YYYYMMDD` or `sync-upstream-pr<N>`.
4. Merge `upstream/main` into that branch.
5. Resolve conflicts locally.
6. Push the sync branch to `origin`.
7. Open or update a PR from the fork-owned sync branch into `origin/main`.

Use this command sequence unless there is a repo-specific reason not to:

```bash
git fetch origin main
git fetch upstream main
git switch -c sync-upstream-YYYYMMDD origin/main
git merge upstream/main
# resolve conflicts
git push -u origin sync-upstream-YYYYMMDD
```

## Conflict Rule

If a sync PR is conflicted and its head branch lives in the upstream repository, do not try to "fix" that PR in place from the fork. Instead:

1. Reproduce the merge on a new fork-owned sync branch.
2. Resolve the conflicts there.
3. Push that branch to the fork.
4. Create a replacement PR from the fork-owned branch.
5. Point reviewers to the replacement PR and close or supersede the old one if appropriate.

## PR Guidance

- Prefer GitHub Copilot MCP first for PR reads and creation when it is available.
- Keep sync PRs as draft PRs until conflicts are resolved.
- Prefer a normal merge commit so the upstream sync point stays explicit.
- Do not merge `upstream/main` directly into `origin/main` locally.
- Use a fresh sync branch each time instead of reusing an older sync branch.

## Validation

Before reporting success:

- Confirm conflict markers are gone.
- Run `git diff --check`.
- Check that the replacement or updated PR is open and points from the fork-owned branch to `main`.

## What To Tell The User

Summarize:

- which sync branch was created
- whether the original PR could be repaired directly
- the replacement PR URL if a new PR was needed
- any remaining CI or review follow-up
