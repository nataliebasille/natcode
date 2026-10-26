# Natcode coding-agent instructions

Before designing or implementing any language feature:

1. Read `docs/README.md`, all relevant files under `docs/spec/`, and the scorecards in `docs/scorecards/` for every system the work touches, including linked items and their open questions. `docs/natcode-spec.md` is a generated single-file copy of the spec.
2. Use each scorecard item's status (Proposed, Open question, Approved, In progress, Implemented) to distinguish decided design from proposals and open questions.
3. Review even apparently complete specifications critically for omissions, contradictions, or unclear behavior. Raise issues rather than making implicit choices.
4. Do **not** invent syntax, semantics, default behavior, or language rules to fill a gap.
5. Do **not** implement an item unless its scorecard status is **Approved** (or already In progress). A **Proposed** item, or an agent's suggested answer to an open question, is not approval.
6. Approved items may be implemented even when related items are still open, as long as they are unaffected by them. Do not broaden an item's scope implicitly.
7. Keep language semantics in `docs/spec/`, and all status, open questions, and test evidence in `docs/scorecards/` (format and rules in `docs/scorecards/README.md`). Undecided subjects are **Open question** items, never spec prose. After editing docs, run `python docs/tools/build_spec.py` and `python docs/tools/scorecard_status.py --check`.
8. Only the language owner moves items to **Open question** or **Approved**, or changes specification decisions. You may add **Proposed** items, add questions, and suggest answers for review. Move items to **In progress** or **Implemented** only for approved work, recording passing tests as evidence.

When blocked, describe the missing decision, why it affects implementation, and possible alternatives without selecting one on the owner's behalf.

## Opening pull requests

Use the GitHub CLI (`gh`) to open pull requests; the GitHub MCP tools cannot create them.

1. **Base the branch on the latest `main`.** Run `git fetch origin` and confirm `git rev-list --left-right --count HEAD...origin/main` reports `0` commits behind; rebase onto `origin/main` otherwise. Check that `main` has not already implemented the same scorecard items before starting work.
2. **Validate.** Run `cargo test`, `cargo clippy --all-targets`, and, if docs changed, `python docs/tools/build_spec.py` and `python docs/tools/scorecard_status.py --check`. Report any failure that already exists on `main` instead of fixing it in an unrelated PR.
3. **Commit and push.** Follow the existing subject-prefix style (`lexer: …`, `docs: …`, `docs(scorecards): …`), then `git push -u origin <branch>`.
4. **Install `gh` if it is missing.** On Windows, run `winget install --id GitHub.cli -e --source winget`. It installs to `C:\Program Files\GitHub CLI\gh.exe`, which shells started before the install will not have on `PATH`, so call it by full path if needed.
5. **Authenticate.** Check `gh auth status`. If `gh` is not logged in, reuse the GitHub token already held by Git Credential Manager, scoped to the current process only:

   ```powershell
   $cred = "protocol=https`nhost=github.com`n`n" | git credential fill
   $env:GH_TOKEN = ($cred | Where-Object { $_ -like 'password=*' }) -replace '^password=', ''
   ```

   Never print, log, or commit the token, and do not persist it with `gh auth login`. The `repo` scope is sufficient for pull requests.
6. **Create the pull request.** Write the description to a temporary file and run `gh pr create --base main --head <branch> --title "<prefix>: <summary>" --body-file <file>`. The description should cover what changed, the scorecard items affected, notes for reviewers, and validation results. Delete the temporary file afterwards.
