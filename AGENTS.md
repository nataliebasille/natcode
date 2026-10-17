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
