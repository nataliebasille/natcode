# Natcore implementation infrastructure scorecard

> **Engineering roadmap, not an approved architecture.** These milestones track necessary compiler and execution capabilities without declaring a particular implementation language, IR design, runtime ABI, or backend architecture mandatory. Adjust milestones if the implementation design changes, but retain completed IDs and evidence.

## Agent completion rules

- Start **every item unchecked** until the repository contains an implementation and the acceptance criterion has an automated passing test. Never infer work is done from this document.
- Mark `- [x]` only after verifying the item; record the **item ID, test command/path, commit or PR** in the evidence table below.
- If a language-level decision is deferred, leave affected tasks unchecked and identify the blocker rather than inventing the behavior.
- Do not count backend output merely being generated as correctness: its output must execute successfully and satisfy behavior tests.
- Read [language feature checklist](../SCORECARD.md), [backend coverage](backend-coverage.md), and `../spec/*.md` together.

### Lexer

- [ ] **INF-LEX-01 — Core lexical tokenization**
  - **Done when:** Automated lexer tests correctly distinguish identifiers, literals, punctuation, operator tokens, strings, comments, and source positions for all relevant approved source forms.
- [ ] **INF-LEX-02 — Unicode and emoji identifiers**
  - **Done when:** Unicode/emoji tokenization, PascalCase and top-level constant naming constraints, and invalid names have lexer/parser test cases.
- [ ] **INF-LEX-03 — Indentation-aware tokenization**
  - **Done when:** Indentation boundaries and malformed indentation have test coverage consistent with the approved indentation-scoped syntax; block grammar details still pending approval remain blockers.

### Parser and diagnostics

- [ ] **INF-PAR-01 — Parse module declaration structure**
  - **Done when:** Tests parse ordered imports, declarations, and exactly one terminal export; invalid file structure is rejected.
- [ ] **INF-PAR-02 — Recoverable syntax errors and locations**
  - **Done when:** Invalid syntax reports spans and recoverable diagnostics in lexer/parser tests without crashing.

### Symbols, modules, and type analysis

- [ ] **INF-RES-01 — Lexical symbol identities and shadowing**
  - **Done when:** Tests resolve immutable redeclarations, capture the correct outer bindings in closures, and distinguish imported symbols from local shadows.
- [ ] **INF-RES-02 — Static module import graph**
  - **Done when:** Resolve relative file paths, imported exports and aliases; report missing files and circular dependency paths with diagnostic tests.
- [ ] **INF-RES-03 — Module interface resolution**
  - **Done when:** Validate known export names/types, namespace re-exports, combined named/namespace imports, missing keys and duplicate imports.
- [ ] **INF-TYPE-01 — Literal and constraint type checking**
  - **Done when:** Literal inference, `:` assignability constraints and distinct immutable binding types pass positive/negative type-check tests.
- [ ] **INF-TYPE-02 — Export-object shape inference**
  - **Done when:** Infer unions of object interfaces and conditional `None`-like missing property types without generating false required properties. Exact `None` representation is still a blocker.
- [ ] **INF-CONST-01 — Sound constant reasoning with fallback**
  - **Done when:** Pure known arithmetic and approved function-local type reasoning infer exact results; exhausted budgets fall back soundly. Detailed budget rules remain deferred.

### Runtime model and native interpreter

- [ ] **INF-RUN-01 — Memoized delayed evaluation**
  - **Done when:** Tests show a demanded thunk evaluates once, shares its result, and unrelated object fields remain suspended.
- [ ] **INF-RUN-02 — Persistent immutable object updates**
  - **Done when:** Tests show dotted update yields a new value, earlier aliases preserve the old value, and unchanged portions can share backing storage. Detailed grammar remains deferred.
- [ ] **INF-INT-01 — Native interpreter executes core programs**
  - **Done when:** A path-selected executable module runs in the native interpreter, returns observable results, and reports runtime errors with usable source locations.
- [ ] **INF-INT-02 — Interpreter supports modules and lazy values**
  - **Done when:** Shared conformance tests for imports, exported namespace objects, memoization, and closure/redeclaration semantics pass in the native interpreter.

### Compiler targets

- [ ] **INF-BE-01 — Target-independent conformance contract**
  - **Done when:** A single behavior-test fixture can be compiled/executed against selected targets and compare normalized output/errors without changing expected Natcore semantics.
- [ ] **INF-JS-01 — JavaScript backend runs core Natcore program**
  - **Done when:** A representative executable module compiles to runnable JS and passes initial shared conformance cases, including collision-safe generated names.
- [ ] **INF-WASM-01 — WebAssembly backend runs core Natcore program**
  - **Done when:** A representative executable compiles to WebAssembly with a runnable harness and passes initial shared conformance cases.
- [ ] **INF-C-01 — C backend runs core Natcore program**
  - **Done when:** Generated C compiles with a supported C toolchain and passes initial shared conformance cases.
- [ ] **INF-PY-01 — Python backend runs core Natcore program (prospective)**
  - **Done when:** If this optional target is adopted, generated Python executes and passes initial shared conformance cases. Leave unchecked while speculative.

### Tests, tooling, and debugger

- [ ] **INF-QA-01 — Unit tests for front-end stages**
  - **Done when:** Lexer, parser, symbol, and type tests run from a documented command in CI or an equivalent repeatable test runner.
- [ ] **INF-QA-02 — Shared language conformance runner**
  - **Done when:** Tests have stable feature IDs, test inputs and expected outcomes; the harness runs against the interpreter and each supported backend independently.
- [ ] **INF-DBG-01 — Lazy-aware debugger foundation**
  - **Done when:** A debug session can show suspended work and explicitly force/step into it without automatically changing ordinary evaluation semantics; relevant tests pass.

## Completion evidence

| Infrastructure item ID | Passing test(s) / command(s) | Commit or PR | Notes / blockers |
| --- | --- | --- | --- |
| — | — | — | Nothing independently verified by this package |

**Note:** Infrastructure tasks measure component readiness; they do not imply all 62 language features are complete or supported by all backends.
