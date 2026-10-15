# Natcore coding-agent instructions

Before designing or implementing any language feature:

1. Read `SPEC.md` and all relevant files under `spec/`, including linked dependencies and unresolved questions.
2. Distinguish authoritative, approved design from proposals, partial specifications, and open questions.
3. Review even apparently complete specifications critically for omissions, contradictions, or unclear behavior. Raise issues rather than making implicit choices.
4. Do **not** invent syntax, semantics, default behavior, or language rules to fill a gap.
5. Do **not** implement a feature or subset unless its scope is explicitly marked **Approved** for implementation by the language owner. An agent's **Recommended** assessment is not approval.
6. Independently specified subsets may be implemented when explicitly approved and unaffected by incomplete related designs. Do not broaden their scope implicitly.
7. Keep language semantics in `spec/`, design alternatives in `design/`, and implementation decisions in `compiler/`.
8. Do not change specification decisions or implementation approvals without explicit owner authorization. You may propose changes and record unresolved questions for review.

When blocked, describe the missing decision, why it affects implementation, and possible alternatives without selecting one on the owner's behalf.
