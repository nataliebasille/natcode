# Types scorecard

- **Prefix:** TYP
- **Covers:** Type inference and checking: literal types, constraints, unions and intersections, function return inference, compile-time evaluation, and module export kinds and shapes.

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
  1. Which simplifications are performed (not just which are forbidden)?
  2. What is the meaning of intersecting incompatible types?

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

## Compile-time evaluation

### TYP-EVAL-01 — Bounded evaluation falls back soundly

- **Status:** Approved
- **Spec:** [evaluation.md — Function return inference: context-independent](../spec/evaluation.md#function-return-inference-context-independent)
- **Done when:** When compile-time evaluation exceeds its budget, inference falls back to a sound static type instead of failing compilation.

### TYP-EVAL-02 — Standardized evaluation budgets

- **Status:** Open question
- **Spec:** [evaluation.md — Function return inference: context-independent](../spec/evaluation.md#function-return-inference-context-independent)
- **Questions:**
  1. What are the exact, deterministic evaluation budgets?
  2. How does compile-time evaluation relate to type checking?

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

### TYP-NONE-01 — Representation of `None`

- **Status:** Open question
- **Spec:** [modules.md — Exported object shapes and optional properties](../spec/modules.md#exported-object-shapes-and-optional-properties)
- **Questions:**
  1. What are the spelling and semantics of `None` (Natcore has no `null` or `undefined`)?
  2. How is it checked against other types?

### TYP-ADT-01 — Algebraic data types and constructors

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Union and intersection types](../spec/syntax-and-types.md#union-and-intersection-types)
- **Questions:**
  1. How are algebraic data types and their constructors designed?

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
