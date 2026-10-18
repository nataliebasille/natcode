# Types scorecard

- **Prefix:** TYP
- **Covers:** Nominal types, constructors and aliases; type inference and checking; set-theoretic union and intersection; function return inference; compile-time evaluation; module export kinds and shapes.

## Nominal types and aliases

### TYP-NOM-01 — Nominal type identity

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** `type UserId(number)` and `type ProductId(number)` introduce distinct identities; values of one are not interchangeable with the other merely because both wrap `number`.

### TYP-NOM-02 — Strict nominal payload boundary

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** A nominal value does not implicitly convert to its payload type or inherit the payload's operators; operator definitions must be explicit when designed.

### TYP-NOM-03 — Multiple positional constructor parameters

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** `type Point(number, number)` allows `Point(10, 20)` and each parameter is typed independently.

### TYP-NOM-04 — Destructuring constructor patterns, no automatic value property

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** Matching `UserId(x)` extracts the contained payload; no standard generated `.value` property is required or assumed.
- **Notes:** Complete match/pattern grammar remains PAR-PAT-01 and PAR-COND-01.

### TYP-NOM-05 — Constraining nominal constructor payloads in type expressions

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** `Value("one" | "two")` as a type describes only values of `Value` with either permitted payload; bare `Value` is unrestricted. `Value(A | B)` equals `Value(A) | Value(B)`, without losing correlations between multiple parameters.
- **Notes:** Full type-expression grammar remains PAR-TYPE-01.

### TYP-NOM-06 — Nest nominal constructor parameters structurally

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** `type Sprint(Position, Color)` takes constructed `Position` and `Color` values as ordinary arguments; writing `Position(number, number)` or `Color(string)` where these do not restrict the declared type has the same parameter meaning.
- **Notes:** No special nominal-combination operator, multiple inheritance, interfaces, classes, or automatic nested-value projection is approved.

### TYP-ALI-01 — Transparent type aliases

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** `alias Identifier = UserId | ProductId` names exactly the union without a new nominal identity. Only `type` creates nominal identities.

### TYP-ALI-02 — Constructor symbol aliases

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** `alias Some = Option.Some` refers to the same constructor and can be used in value construction and matching without introducing a new nominal identity.

## Literal precision

### TYP-LIT-01 — Exact primitive literal types

- **Status:** Approved
- **Spec:** [variables.md — Exact type inference](../spec/variables.md#exact-type-inference)
- **Done when:** Bindings initialized with `42`, `"Alice"`, and `true` have the literal types `42`, `"Alice"`, and `true`.

### TYP-LIT-02 — Exact constant-expression types

- **Status:** Approved
- **Spec:** [evaluation.md — Literal precision](../spec/evaluation.md#literal-precision)
- **Done when:** Pure constant arithmetic such as `40 + 2` infers the literal type `42` when evaluation succeeds.

## Constraints

### TYP-CON-01 — Constraints check assignability

- **Status:** Approved
- **Spec:** [variables.md — Declaration syntax](../spec/variables.md#declaration-syntax)
- **Done when:** `x: number = 42` passes and `x: string = 42` fails.

### TYP-CON-02 — Constraints never widen

- **Status:** Approved
- **Spec:** [variables.md — Declaration syntax](../spec/variables.md#declaration-syntax)
- **Done when:** After `x: number = 42`, `x` still has the inferred type `42`.

### TYP-CON-03 — Assignability and subtyping rules

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Union and intersection types](../spec/syntax-and-types.md#union-and-intersection-types)
- **Questions:**
  1. What are the assignability and subtype relations between types?
  2. Is subtyping semantic (set-based) or structural, and how are semantic subtyping details defined?

## Unions and intersections

### TYP-UNI-01 — Union types

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Union and intersection types](../spec/syntax-and-types.md#union-and-intersection-types)
- **Done when:** `A | B` is checked as a union type, distinct from `:` constraints.

### TYP-UNI-02 — Intersection types

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Union and intersection types](../spec/syntax-and-types.md#union-and-intersection-types)
- **Done when:** `A & B` is checked as an intersection type, distinct from `:` constraints.

### TYP-UNI-03 — Correlation-preserving union simplification

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Union and intersection types](../spec/syntax-and-types.md#union-and-intersection-types)
- **Done when:** A discriminated record union is never simplified into one that admits cross-combinations of fields absent from the original.

### TYP-UNI-04 — Detailed union and intersection semantics

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Union and intersection types](../spec/syntax-and-types.md#union-and-intersection-types)
- **Questions:**
  1. Beyond the approved mathematical identities, which normalizations and simplifications are performed?
  2. What are the precise membership and overlap rules for structural records and future advanced type forms?

### TYP-UNI-06 — Set-theoretic union and intersection identities

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Union and intersection types](../spec/syntax-and-types.md#union-and-intersection-types)
- **Done when:** `A | B` and `A & B` denote set union and intersection respectively, with commutativity and idempotence; neither operator creates a new nominal identity or constructs a value.

### TYP-UNI-07 — Universal `Any` and empty `Empty` types

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Union and intersection types](../spec/syntax-and-types.md#union-and-intersection-types)
- **Done when:** `Any` is universal and `Empty` has no inhabitants; `A | Empty = A`, `A & Empty = Empty`, `A | Any = Any`, and `A & Any = A`.

### TYP-UNI-08 — Empty intersections are valid expressions

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Union and intersection types](../spec/syntax-and-types.md#union-and-intersection-types)
- **Done when:** Provably empty intersections, including nested expressions, resolve to `Empty` without an intersection-specific error or warning; attempts to supply impossible values are rejected.

### TYP-UNI-09 — Unrelated nominal constructors are disjoint

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Union and intersection types](../spec/syntax-and-types.md#union-and-intersection-types)
- **Done when:** `UserId & ProductId` resolves to `Empty` for unrelated nominal constructor types, even when they have the same payload representation.

### TYP-UNI-05 — Difference and complement types

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Union and intersection types](../spec/syntax-and-types.md#union-and-intersection-types)
- **Questions:**
  1. Are there difference or complement type operators, and with what syntax?

## Functions

### TYP-FN-01 — Context-independent return inference

- **Status:** Approved
- **Spec:** [evaluation.md — Function return inference: context-independent](../spec/evaluation.md#function-return-inference-context-independent)
- **Done when:** With a parameter of type `number`, `x * 2` infers `number`; with a parameter of literal type `24`, it infers `48`, regardless of call-site values.

### TYP-FN-02 — Exact types through function-local declarations

- **Status:** Approved
- **Spec:** [evaluation.md — Function return inference: context-independent](../spec/evaluation.md#function-return-inference-context-independent)
- **Done when:** A function with parameter `x: 24` and a local `multiplier = 2` infers the literal return type `48`.

### TYP-FN-03 — Overloading

- **Status:** Open question
- **Spec:** [evaluation.md — Function return inference: context-independent](../spec/evaluation.md#function-return-inference-context-independent)
- **Questions:**
  1. Are function overloads supported, and how are they declared and selected?

### TYP-FN-04 — Detailed return-inference rules

- **Status:** Open question
- **Spec:** [evaluation.md — Function return inference: context-independent](../spec/evaluation.md#function-return-inference-context-independent)
- **Questions:**
  1. What are the detailed rules and precision limits of context-independent return inference, including for recursive functions?

### TYP-FN-05 — Preserving a containing value across nested transformations

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Questions:**
  1. How can a function express that a returned nested value updates the corresponding part of an input's containing value while preserving the remainder?
  2. How does that differ from a function that produces a new, unrelated nested value?
- **Notes:** Deferred until the function system is designed; ordinary constructor nesting implies neither behavior.

## Compile-time evaluation

### TYP-EVAL-01 — Optional evaluation preserves type semantics

- **Status:** Approved
- **Spec:** [evaluation.md — Function return inference: context-independent](../spec/evaluation.md#function-return-inference-context-independent)
- **Done when:** Optional evaluation success or failure cannot affect inferred types, runtime semantics or compilation validity.

### TYP-EVAL-02 — Guaranteed evaluation subset and budgets

- **Status:** Open question
- **Spec:** [evaluation.md — Function return inference: context-independent](../spec/evaluation.md#function-return-inference-context-independent)
- **Questions:**
  1. What deterministic evaluation capabilities and resource budgets are guaranteed?
  2. How are resource-limit failures handled for mandatory checks?

### TYP-EVAL-03 — Compile-time pure-function validation

- **Status:** Approved
- **Spec:** [evaluation.md — Function return inference: context-independent](../spec/evaluation.md#function-return-inference-context-independent)
- **Done when:** Eligible pure functions with known inputs validate mandatory type constraints; false constraints are compile errors, without silent broadening or fallback.

### TYP-EVAL-04 — Constraint grammar and unknown inputs

- **Status:** Open question
- **Spec:** [evaluation.md — Function return inference: context-independent](../spec/evaluation.md#function-return-inference-context-independent)
- **Questions:**
  1. What is the constraint declaration syntax, including validation of entire constructor values?
  2. Which pure functions qualify and how are unknown inputs handled?

### TYP-EVAL-05 — Symbolic reasoning and proofs

- **Status:** Open question
- **Spec:** [evaluation.md — Function return inference: context-independent](../spec/evaluation.md#function-return-inference-context-independent)
- **Questions:**
  1. Should the compiler reason about constraints over unknown values, beyond evaluation?

### TYP-DIAG-01 — Compile errors versus warnings

- **Status:** Approved
- **Spec:** [evaluation.md — Function return inference: context-independent](../spec/evaluation.md#function-return-inference-context-independent)
- **Done when:** Missing or disproven mandatory guarantees cause errors; known redundancy in valid code can warn without blocking compilation; optional analysis failure alone has no diagnostic.

### TYP-PAT-01 — Exhaustiveness and redundant catch-all

- **Status:** Approved
- **Spec:** [syntax-and-types.md — When expressions](../spec/syntax-and-types.md#when-expressions)
- **Done when:** Subject-based and subjectless `when` require `_` unless remaining possibilities are provably `Empty`; a provably redundant `_` warns but is valid, and arbitrary predicates alone do not prove exhaustiveness.

## Module export kinds and shapes

### TYP-EXP-01 — Export kind classification

- **Status:** Approved
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** An exported object makes a library module and an exported function makes an executable module, decided statically.

### TYP-EXP-02 — Invalid export kinds rejected

- **Status:** Approved
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** Directly exporting a primitive or a type is rejected; the same inside an exported object is accepted.

### TYP-EXP-03 — Arbitrary export expressions of provable kind

- **Status:** Approved
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** Exporting a function call is accepted when its inferred result is provably object-only or function-only.

### TYP-EXP-04 — Ambiguous export kind rejected

- **Status:** Approved
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** An export expression whose type may be either an object or a function is a compile error.

### TYP-EXP-05 — Statically enumerated export keys

- **Status:** Approved
- **Spec:** [modules.md — Exported object shapes and optional properties](../spec/modules.md#exported-object-shapes-and-optional-properties)
- **Done when:** Union-shaped export objects contribute every possible key; a top-level export with an unenumerable key set is rejected.

### TYP-EXP-06 — Conditionally present export keys include absence

- **Status:** Approved
- **Spec:** [modules.md — Exported object shapes and optional properties](../spec/modules.md#exported-object-shapes-and-optional-properties)
- **Done when:** A key present in only some export shapes is importable, and its inferred type includes a `None`-like case.
- **Notes:** The representation of `None` is TYP-NONE-01.

## Open type-system subjects

### TYP-ERR-01 — Error designation and propagation details

- **Status:** Open question
- **Spec:** [evaluation.md — Error handling](../spec/evaluation.md#error-handling)
- **Questions:**
  1. How does a type receive the error designation (declaration syntax, marker type, protocol, or another mechanism)?
  2. How are possible errors reflected in type inference, union types, and function return types?
  3. How do callers handle, transform, and propagate error values without exceptions?
- **Notes:** Explicit, unwrapped error values and compiler-recognized designated error types are approved in TYP-ERR-02. Language-level exceptions are rejected (RUN-EFF-04). **Defer all remaining error designation, typing, and propagation decisions until the foundational type model is worked through**, especially TYP-ADT-02 (remaining algebraic type questions), TYP-CON-03 (subtyping), and PAR-TYPE-02 (remaining declaration grammar). Do not assume marker types, structural inheritance, or nominal branding.

### TYP-ERR-02 — Designated first-class error types

- **Status:** Approved
- **Spec:** [evaluation.md — Error handling](../spec/evaluation.md#error-handling)
- **Done when:** Error values are ordinary first-class values; success values and designated error values may be returned directly as alternatives in a union, without obligatory `Ok`/`Err` wrappers. The type checker recognizes designated error types as distinct from ordinary non-error types.
- **Notes:** The mechanism of designating an error type, exact return-type inference, and propagation rules remain open in TYP-ERR-01.

### TYP-NONE-01 — Representation of `None`

- **Status:** Open question
- **Spec:** [modules.md — Exported object shapes and optional properties](../spec/modules.md#exported-object-shapes-and-optional-properties)
- **Questions:**
  1. What are the spelling and semantics of `None` (Natcode has no `null` or `undefined`)?
  2. How is it checked against other types?

### TYP-ADT-01 — Nominal algebraic types with constructors

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** `type Option` with indented `Some(any)` and `None` introduces a nominal sum-type and constructors accessible as `Option.Some` and `Option.None`; bare unqualified constructors are not introduced automatically.

### TYP-ADT-02 — Remaining algebraic type semantics

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Questions:**
  1. What further rules govern generic, recursive, nested and other advanced constructor definitions?
  2. What are the exact variant narrowing and exhaustiveness rules beyond approved constructor pattern extraction?

### TYP-GEN-01 — Generics

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Union and intersection types](../spec/syntax-and-types.md#union-and-intersection-types)
- **Questions:**
  1. Are there generic types and functions, and how are type parameters declared and inferred?

### TYP-NS-01 — Type values inside namespaces

- **Status:** Open question
- **Spec:** [modules.md — First-class namespaces and persistent updates](../spec/modules.md#first-class-namespaces-and-persistent-updates)
- **Questions:**
  1. How are types represented at runtime when they are members of a first-class namespace value?
