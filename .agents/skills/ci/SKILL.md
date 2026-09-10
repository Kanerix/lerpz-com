---
name: ci
description: Investigate a GitHub Actions run for this repository with the gh CLI. Use when a pipeline run fails, when a deploy did not happen, or when a task refers to a run, pull request or issue you cannot see.
---

# Investigating a CI run

The workflows are in `.github/workflows/`. Read the ones involved in the run you
are looking at. Do not describe the pipeline from memory or from this file, the
workflows are the source of truth and they change.

## Find the run

```sh
gh run list --limit 5
gh run list --workflow=pipeline.yaml --branch=dev
gh run list --commit=<sha>
```

Do not use `gh run watch`. It blocks until the run finishes and will sit there
until the command times out.

## Read the failure

```sh
gh run view <id>
gh run view <id> --log-failed
```

`--log-failed` prints only the failing steps and is what you want first. Reach
for the full `--log` only when the failing step alone does not explain itself,
and expect it to be long.

`gh run view <id> --json jobs` gives the job and step names, conclusions and
`startedAt` values if you need to pick them apart precisely.

## Establish what actually happened

Work out which job failed, then open the workflow file that defines it and read
the step. A failure only makes sense next to the step that produced it.

A job that did not run is not a job that failed. Before concluding that
something was missed, check the job's `if:` condition, its `needs:`, and any
outputs those dependencies produce. Skipping can be the correct outcome for the
branch or the set of changed paths.

## Reproduce it locally

Find the `run:` command in the failing step and run the same command yourself,
through `devenv shell -- <cmd>`. If the step uses an action rather than a
command, read what the action does and find the local equivalent.

Fix the cause in the repository. Do not change a workflow to make a failure go
away unless the workflow is the thing that is wrong.

## Pull request and issue context

When a task refers to something you cannot see, read it rather than asking for a
paste:

```sh
gh pr view <number>
gh pr diff <number>
gh issue view <number>
```

## If gh cannot authenticate

Never run `gh auth login` or `gh auth refresh` yourself. Both are interactive and
will block until the command times out. Ask the user to run them in their own
terminal, and wait.

Diagnose first, which is safe and non-interactive:

```sh
gh auth status
```

It reports the logged in account, the token type and the token scopes.

If it reports no account, stop and ask the user to run `gh auth login`.

If a command needs a scope that `gh auth status` does not list, ask the user to
run `gh auth refresh -h github.com -s <scope>` with the scope named in the
error.

SSO is the case worth recognising, because it does not look like an auth
problem. If the repository, or something the task references, lives in an
organisation that enforces SAML SSO, `gh` logs in fine and then fails on the
actual request with a 403 whose message mentions SAML enforcement. The login is
not broken. The token exists but has not been authorised for that organisation,
which is a separate step. Ask the user to either open the URL included in the
error and authorise the token for the organisation, or run `gh auth login` again
and complete the SSO prompt in the browser. Expect the same thing on a
repository that has moved into an organisation, where commands that used to work
start returning 403.

In every case, say which command failed and quote the error, then stop. Do not
guess at the contents of a run, a pull request or an issue, and do not fall back
to fetching the web UI, which will not show private data anyway.
