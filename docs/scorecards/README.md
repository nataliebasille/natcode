# Natcode scorecards

Each system of the Natcode toolchain has one scorecard. A scorecard lists every **atomic item** that system must implement, and records each item's decision and implementation status, its open questions, and its test evidence.

The scorecards are the **only** record of status. Spec files in [`../spec/`](../spec/) describe semantics and refer to scorecard items by ID; they don't carry their own status labels or open-question lists.

## Systems

| Scorecard | Prefix | Covers |
| --- | --- | --- |
| [lexer.md](lexer.md) | `LEX` | Tokens, identifiers, comments, whitespace, indentation |
| [parser.md](parser.md) | `PAR` | Grammar, module file structure, syntax errors |
| [resolver.md](resolver.md) | `RES` | Names, scopes, shadowing, closures, module/import graph, export interfaces |
| [types.md](types.md) | `TYP` | Literal inference, constraints, unions/intersections, constant evaluation, export kinds |
| [runtime.md](runtime.md) | `RUN` | Lazy evaluation, memoization, namespaces as values, persistent updates, effects |
| [interpreter.md](interpreter.md) | `INT` | Native interpreter execution and conformance |
| [backend-js.md](backend-js.md) | `JS` | JavaScript code generation and conformance |
| [backend-wasm.md](backend-wasm.md) | `WASM` | WebAssembly code generation and conformance |
| [backend-c.md](backend-c.md) | `C` | C code generation and conformance |
| [backend-python.md](backend-python.md) | `PY` | Python code generation and conformance (prospective) |
| [debugger.md](debugger.md) | `DBG` | Lazy-aware debugging |
| [tooling.md](tooling.md) | `TOOL` | Test harness, conformance runner, CLI, build system, shared architecture |

## Statuses

| Status | Meaning |
| --- | --- |
| **Proposed** | Suggested by an agent; not yet accepted by the language owner. |
| **Open question** | Behavior is undecided. The item lists its questions. |
| **Approved** | Decided by the language owner and ready to implement. |
| **In progress** | Being implemented. |
| **Implemented** | Meets the current approved specification, with passing automated tests recorded as evidence. |
| **Needs update** | Previously implemented with evidence, but an approved specification change requires implementation and/or test updates. |
| **Superseded** | Retired or replaced behavior, kept as a historical record; not an active implementation target. |

### Transitions

```
Proposed ──► Open question ──► Approved ──► In progress ──► Implemented
    │                             ▲                             │
    └─────────────────────────────┘                             │
                                  In progress ◄── Needs update ◄─┘

Any active status ──► Superseded (with language-owner approval)
```

- Agents add new items as **Proposed**. Only the language owner moves an item from Proposed to Open question or Approved.
- **Open question** moves only to **Approved**, when the owner answers it. Agents may write a suggested answer under a question, but must not change the status.
- Postponed subjects stay **Open question**; there is no separate deferred state.
- Agents may move an item from **Approved** to **In progress**, and to **Implemented** once its tests pass.
- When an **Implemented** item's approved requirements change, move it to **Needs update**, replace its *Done when* criterion with the new one, and move old *Evidence* to *Previous evidence*. It returns through **In progress** to **Implemented** only with passing tests for the new requirement.
- **Superseded** is a terminal historical status for behavior that has been retired or replaced entirely; preserve its ID, record the replacement ID (or `None` if removed outright), and explain why in Notes. Language-owner approval is required to supersede an item.
- Keep *Previous evidence* on the item even after its updated implementation returns to **Implemented**. These references are historical, so a later removal or rename of the old test does not invalidate the scorecard.
- Work may begin on **Approved** or **Needs update** items. Do not treat **Needs update** or **Superseded** as implemented current behavior.
- Resolve one open question at a time, and keep decisions distinct from illustrative code in the spec.

## Item format

```markdown
### LEX-NUM-02 — Digit separators between digits

- **Status:** Implemented
- **Spec:** [lexical.md — Number rules](../spec/lexical.md#number-rules)
- **Done when:** `1_000` lexes as one number; `1__0`, `1_`, `1_.5`, `1._5`, `1.5_` are errors.
- **Evidence:** `tests/lexer/main.rs::numbers::digit_separators_between_digits`
- **Notes:** Optional free text.
```

| Field | Required | Content |
| --- | --- | --- |
| Heading | Always | `### <ID> — <short title>` |
| **Status** | Always | One of the seven statuses. |
| **Spec** | Always | Link to the spec section, or `None` for engineering items with no language rule. |
| **Done when** | Unless Open question or Superseded | The current observable acceptance criterion; Superseded items may retain their former one. |
| **Evidence** | Implemented only | Passing tests for the current specification, backticked `path::module::test_name` references. |
| **Previous evidence** | Needs update required; In progress, Implemented, Superseded optional | Historical, backticked test references from an older implementation; these may reference removed tests. |
| **Superseded by** | Superseded only | Replacement scorecard item ID, or `None` if retired without replacement. |
| **Questions** | Open question only | A numbered list of the undecided points. |
| **Notes** | Superseded required, otherwise optional | Context, blockers, revision history, or reason for retirement. |

## IDs

IDs have the form `<PREFIX>-<AREA>-<NN>`, for example `LEX-NUM-02` or `RES-SCOPE-04`. The prefix matches the scorecard; the area groups related items within it. IDs are never renumbered or reused, so links and evidence stay valid. Keep obsolete items as **Superseded** rather than deleting them.

## Checking status

From the repository root:

```
python docs/tools/scorecard_status.py          # status counts per system
python docs/tools/scorecard_status.py --json   # machine-readable counts
python docs/tools/scorecard_status.py --check  # validate format, links, evidence, and the merged spec
```

`--check` verifies that every item has the required fields for its status, IDs are unique and match their file's prefix, spec links resolve, current evidence names existing tests, historical evidence is recorded separately, replacement IDs exist, IDs referenced from spec files exist, and [`../natcode-spec.md`](../natcode-spec.md) matches the spec files. Regenerate the merged spec with `python docs/tools/build_spec.py`.
