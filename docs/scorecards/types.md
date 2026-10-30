# Types scorecard

- **Prefix:** TYP
- **Covers:** Language-level type semantics and their required behavior: nominal types, constructors and aliases; type inference and checking rules; set-theoretic union and intersection; function return inference; compile-time evaluation; module export kinds and shapes. Checker integration and implementation tracking live in [typechecker.md](typechecker.md).

## Nominal types and aliases

### TYP-NOM-01 — Nominal type identity

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** `unique type UserId(Number)` and `unique type ProductId(Number)` introduce distinct identities; values of one are not interchangeable with the other merely because both wrap `Number`.

### TYP-NOM-02 — Strict nominal payload boundary

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** A nominal value does not implicitly convert to its payload type or inherit the payload's operators; operator definitions must be explicit when designed.

### TYP-NOM-03 — Multiple positional constructor parameters

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** `unique type Point(Number, Number)` allows `Point(10, 20)` and each parameter is typed independently.

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
- **Done when:** `unique type Sprint(Position, Color)` takes constructed `Position` and `Color` values as ordinary arguments; writing `Position(Number, Number)` or `Color(String)` where these do not restrict the declared type has the same parameter meaning.
- **Notes:** No special nominal-combination operator, multiple inheritance, interfaces, classes, or automatic nested-value projection is approved.

### TYP-NOM-07 — Specialized constructor aliases expose remaining parameters

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Specialized aliases and constructors](../spec/syntax-and-types.md#specialized-aliases-and-constructors)
- **Done when:** `type XAxis = Position(Number, 0)` exposes one constructor parameter and `XAxis(10)` constructs `Position(10, 0)`; `Position(0, 0)` exposes zero parameters.

### TYP-NOM-08 — Optional parentheses for zero-parameter application

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Specialized aliases and constructors](../spec/syntax-and-types.md#specialized-aliases-and-constructors)
- **Done when:** Zero-parameter applications may omit `()` in both type and value contexts, so `Center` equals `Center()` in type context and nullary constructors such as `Option.None` and `Option.None()` are equivalent in value context.

### TYP-ALI-01 — Transparent type aliases

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** `type Identifier = UserId | ProductId` names exactly the union without a new nominal identity. Only `unique type` creates nominal identities.

### TYP-ALI-02 — Constructor symbol aliases

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** `type Some = Option.Some` refers to the same constructor and can be used in value construction and matching without introducing a new nominal identity.

## Refinement types

### TYP-REF-01 — `where` defines a transparent subset

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Refinement types with `where`](../spec/syntax-and-types.md#refinement-types-with-where)
- **Done when:** `T where pattern => predicate` denotes exactly the values of `T` selected by the Boolean predicate and creates no new nominal identity.

### TYP-REF-02 — Parameter and constructor restrictions

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Restrictions in nominal declarations](../spec/syntax-and-types.md#restrictions-in-nominal-declarations)
- **Done when:** A `where` may restrict one constructor parameter or a single whole nominal constructor, including constructors of algebraic types; parameter-level restrictions see only that parameter, whole-constructor restrictions may relate parameters, and every applicable restriction must hold. A nominal declaration containing multiple constructors cannot receive a single declaration-wide `where`; alias-level filtering of an existing algebraic type remains available.

### TYP-REF-03 — Non-exhaustive filtering branches

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Where patterns and filtering](../spec/syntax-and-types.md#where-patterns-and-filtering)
- **Done when:** Unmatched values are excluded without requiring exhaustiveness; ordinary branch patterns may not overlap; a matched value whose predicate is false is excluded without falling through; `_` denotes only the remaining unmatched region; explicit true/false branches and bare-constructor inclusion sugar are supported.

### TYP-REF-04 — Restricted aliases preserve surviving constructors

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Specialized aliases and constructors](../spec/syntax-and-types.md#specialized-aliases-and-constructors)
- **Done when:** A restricted algebraic alias exposes each underlying constructor whose remaining set is not proven `Empty`, removes constructors proven empty, enforces the alias restriction during construction, and does not invent a direct constructor that the original type did not have.

### TYP-REF-05 — Constructibility follows the normalized result

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Constructibility through set algebra](../spec/syntax-and-types.md#constructibility-through-set-algebra)
- **Done when:** Set operations preserve or produce direct construction only when the normalized result has one effective constructor shape; `XAxis | YAxis` is not directly callable when their free positions differ, while `XAxis & YAxis` may normalize to zero-parameter `Position(0, 0)`.

### TYP-REF-06 — Refinement predicates are pure and deterministic

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Purity and lexical capture](../spec/syntax-and-types.md#purity-and-lexical-capture)
- **Done when:** Type membership cannot depend on I/O, mutable state, time, randomness, network/database state, or other effects; eligible pure helper functions may be used.

### TYP-REF-07 — Empty refinements are valid

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Where patterns and filtering](../spec/syntax-and-types.md#where-patterns-and-filtering)
- **Done when:** A restriction that removes every inhabitant resolves to `Empty` without an error, and filtering `Empty` also resolves to `Empty`.

### TYP-REF-08 — Restricted facts survive destructuring and exhaustiveness

- **Status:** Approved
- **Spec:** [evaluation.md — Flow-sensitive refinement and symbolic reasoning](../spec/evaluation.md#flow-sensitive-refinement-and-symbolic-reasoning)
- **Done when:** Destructuring a restricted value preserves applicable restrictions on extracted values, and `when` exhaustiveness does not require constructors already eliminated from the subject type.

### TYP-REF-09 — Flow-sensitive refinement

- **Status:** Approved
- **Spec:** [evaluation.md — Flow-sensitive refinement and symbolic reasoning](../spec/evaluation.md#flow-sensitive-refinement-and-symbolic-reasoning)
- **Done when:** True and false control-flow paths refine bindings from direct Boolean conditions, Boolean composition, previous failed branches, and pure named predicates whose facts are known to the compiler.

### TYP-REF-10 — Initial symbolic solver supports linear numeric reasoning

- **Status:** Approved
- **Spec:** [evaluation.md — Flow-sensitive refinement and symbolic reasoning](../spec/evaluation.md#flow-sensitive-refinement-and-symbolic-reasoning)
- **Done when:** The guaranteed solver handles equality/order relations, Boolean combinations, transitive implications, and linear arithmetic with known constant coefficients; nonlinear relationships between unknown values are valid predicates but need not be symbolically proven.

### TYP-REF-18 — Refining complemented types

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Where placement and precedence](../spec/syntax-and-types.md#where-placement-and-precedence)
- **Done when:** `where` can restrict a complement type such as `~Number` under the same pattern-narrowing and operand typing rules as any other type. Numeric operations on a whole `~Number` candidate are invalid, but a nominal constructor pattern can narrow the candidate and expose a numeric payload for valid comparisons. `~Number where ...` restricts the complement; `~(Number where ...)` complements the refinement instead.
- **Notes:** Approved 2026-10-25. No new special filtering syntax.

### TYP-REF-11 — Restricted assignability requires established membership

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Refinement types with `where`](../spec/syntax-and-types.md#refinement-types-with-where)
- **Done when:** A restricted type is assignable to the type it restricts; a value of the broader type is accepted as the restricted type only when compile-time evaluation, existing refinement facts, or the supported solver establishes membership.

### TYP-REF-12 — Explicit runtime validation with `as?`

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Runtime validation](../spec/syntax-and-types.md#runtime-validation)
- **Done when:** For a runtime-checkable restricted target `T` with one compatible non-`Empty` validation input type `U`, `x as? T` is accepted only when the static type of `x` is assignable to `U`; success returns the original value typed as `T`, while failure returns designated `ValidationError`. `as?` does not convert, construct, or dynamically discriminate an unrelated underlying type.
- **Notes:** The right operand is a complete type expression (PAR-VAL-01). A target whose validation input type is `Empty`, or which has no single compatible validation input type, is rejected. Pattern matching is the intended mechanism for discriminating unrelated alternatives.

### TYP-REF-13 — Stable validation signature

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Runtime validation](../spec/syntax-and-types.md#runtime-validation)
- **Done when:** Every valid `x as? T` has result type `T | ValidationError`, even when compile-time evaluation or symbolic reasoning proves that the particular application must succeed or fail; such proof may remove runtime work or precompute the result but does not collapse the expression type.

### TYP-REF-14 — Validation across compatible restricted subsets

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Runtime validation](../spec/syntax-and-types.md#runtime-validation)
- **Done when:** Runtime validation accepts `where` refinements, constructor-payload restrictions, sibling refinements with the same validation input type, inherited/nested refinement chains, and compatible union/intersection/difference targets. A union succeeds if any alternative accepts the value; an intersection requires every component; difference uses ordinary set-membership semantics.

### TYP-REF-15 — Validation failure aggregation

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Runtime validation](../spec/syntax-and-types.md#runtime-validation)
- **Done when:** A failed `as?` produces a designated `ValidationError` that captures every determinable applicable type-level validation failure. Independent checks do not stop after the first failure; union-branch failures are discarded when any union alternative succeeds and are aggregated only when all alternatives fail.

### TYP-REF-16 — Validation-error payloads and type-level messages

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Runtime validation](../spec/syntax-and-types.md#runtime-validation)
- **Questions:**
  1. What is the exact data/constructor shape of `ValidationError`?
  2. How can a developer attach human-readable messages, machine-readable codes, paths, or other metadata to type-level validation rules?
  3. How are failures from nested and composite restrictions represented without losing their originating type-level errors?
- **Notes:** `ValidationError` being a designated error and aggregating determinable failures is already approved in TYP-REF-12 and TYP-REF-15.

### TYP-REF-17 — Validation composition and error short-circuiting

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Runtime validation](../spec/syntax-and-types.md#runtime-validation)
- **Questions:**
  1. How do consecutive validations compose, especially through future pipe syntax?
  2. Should pipes or another construct short-circuit designated error values automatically, and if so under what typing rules?
- **Notes:** General pipe syntax remains PAR-PIPE-01; this item does not approve a pipe operator.

## Literal precision

### TYP-LIT-01 — Exact primitive literal types

- **Status:** Approved
- **Spec:** [variables.md — Exact type inference](../spec/variables.md#exact-type-inference)
- **Done when:** Bindings initialized with `42`, `"Alice"`, and `true` have the literal types `42`, `"Alice"`, and `true`.

### TYP-LIT-02 — Exact constant-expression types

- **Status:** Approved
- **Spec:** [evaluation.md — Literal precision](../spec/evaluation.md#literal-precision)
- **Done when:** Supported core constant arithmetic such as `40 + 2` infers literal type `42` as a deterministic typing rule, independent of optional optimization.

## Constraints

### TYP-CON-01 — Constraints check assignability

- **Status:** Approved
- **Spec:** [variables.md — Declaration syntax](../spec/variables.md#declaration-syntax)
- **Done when:** `x: Number = 42` passes and `x: String = 42` fails.

### TYP-CON-02 — Constraints never widen

- **Status:** Approved
- **Spec:** [variables.md — Declaration syntax](../spec/variables.md#declaration-syntax)
- **Done when:** After `x: Number = 42`, `x` still has the inferred type `42`.

### TYP-CON-03 — Assignability and subtyping rules

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Union and intersection types](../spec/syntax-and-types.md#union-and-intersection-types)
- **Questions:**
  1. What are the assignability and subtype relations between types?
  2. Is subtyping semantic (set-based) or structural, and how are semantic subtyping details defined?

## Unions and intersections

### TYP-COMP-01 — Set-based type-level computation

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Type-expression computations and record syntax](../spec/syntax-and-types.md#type-expression-computations-and-record-syntax)
- **Done when:** Arithmetic, comparison, and strict Boolean operations within type expressions denote sets of possible results, distributing over independent finite literal alternatives while retaining symbolic constraints over larger numeric types when provable. Exact literal calculations such as `10 * (2 | 30)` yield `20 | 300`, while `Number + 1` denotes all possible results of the operation subject to approved numeric semantics.
- **Notes:** Numeric representation, rounding, precision, and unrestricted symbolic arithmetic limits remain TYP-BLT-05/06 and TYP-EVAL-02.

### TYP-COMP-02 — Correlated binding computations

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Type-expression computations and record syntax](../spec/syntax-and-types.md#type-expression-computations-and-record-syntax)
- **Done when:** Type-level computations use the inferred type of a referenced value binding, including unions; repeated uses of one binding denote the same value choice, and derived bindings preserve that dependency. `x + x` for one `x: 1 | 2` yields `2 | 4`; with `y = x + 1`, `x + y` yields `3 | 5`. Independent literal-union operands may combine independently.
- **Notes:** Approved 2026-10-25. Runtime object correlations and larger solver/representation strategies are implementation design work.

### TYP-COMP-03 — Symbolic arithmetic on refinements

- **Status:** Approved
- **Spec:** [evaluation.md — Flow-sensitive refinement and symbolic reasoning](../spec/evaluation.md#flow-sensitive-refinement-and-symbolic-reasoning)
- **Done when:** Inferred return/result types preserve provable symbolic arithmetic constraints on refined values rather than unconditionally widening to their broad numeric base types. For exact arithmetic, adding 1 to `Number where x => x > 0` gives the set satisfying `x > 1`; actual equivalences must respect Number's future representation and rounding semantics.
- **Notes:** The guaranteed solver boundary is TYP-REF-10; exact numeric behavior remains TYP-BLT-05/06.

### TYP-BOOL-01 — Strict Boolean operations and no truthiness

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Strict Boolean operations](../spec/syntax-and-types.md#strict-boolean-operations)
- **Done when:** `&&`, `||`, and `!` accept exclusively Boolean operand types and yield only Boolean results, preserving literal `true`/`false` when derivable. Any possible non-Boolean operand is a compile error, not a silently excluded possibility. No implicit truthiness exists. Subjectless `when` conditions and `where` predicates require Boolean results. The same typing applies at runtime and in type expressions.
- **Notes:** Runtime short-circuit and Boolean-only results are RUN-BOOL-01; approved 2026-10-25.

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

### TYP-UNI-05 — Set complement with prefix `~`

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Union and intersection types](../spec/syntax-and-types.md#union-and-intersection-types)
- **Done when:** `~A` denotes `Any \ A`, the mathematical complement relative to `Any`, and does not mean Boolean negation. Prefix `!` is reserved for strict Boolean NOT.
- **Notes:** Formerly an open complement-operator question; prefix `~` approved 2026-10-25.

### TYP-UNI-10 — Set difference with `\`

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Union and intersection types](../spec/syntax-and-types.md#union-and-intersection-types)
- **Done when:** `A \ B` denotes values in `A` that are not in `B`; normalization may preserve constructibility when the resulting type has one effective constructor shape. Arithmetic division continues to use `/`, including within type expressions.
- **Notes:** Replaces previously approved `/` difference spelling as of 2026-10-25.

## Functions

### TYP-FN-01 — Context-independent symbolic return inference

- **Status:** Approved
- **Spec:** [evaluation.md — Function return inference: context-independent](../spec/evaluation.md#function-return-inference-context-independent)
- **Done when:** A function is analyzed independently of call sites to infer a symbolic return type involving its implicitly polymorphic parameter types, then its signature is instantiated with each argument's inferred type. The return type preserves exact literals, parameter relationships, branch conditions, object properties, and supported arithmetic effects; for example `fn increment(value: Number) { value + 1 }` called with literal 41 has result type 42.
- **Notes:** Revises the previously approved fixed-parameter return-type widening behavior while preserving independent function analysis; advanced precision limits and recursion remain TYP-FN-04.

### TYP-FN-06 — Implicit independently quantified function parameters

- **Status:** Approved
- **Spec:** [evaluation.md — Function return inference: context-independent](../spec/evaluation.md#function-return-inference-context-independent)
- **Done when:** Each function parameter is implicitly a separate polymorphic type variable, constrained by its optional written annotation or by `Any` if absent. A function can preserve the exact call-site argument type through return values, records, and branches even when other parameters share the same annotation; annotations constrain accepted types without widening them.
- **Notes:** No explicit generic-parameter syntax is implied. Parameter annotation grammar is PAR-FN-04.

### TYP-FN-07 — First-class function type semantics

- **Status:** Open question
- **Spec:** [evaluation.md — Function return inference: context-independent](../spec/evaluation.md#function-return-inference-context-independent)
- **Questions:**
  1. What values belong to a function type, and what are the assignability and subtype rules between function parameter and result types, including parameter labels?
  2. How are implicitly polymorphic function values represented and instantiated when assigned, passed as arguments, returned, or constrained by a written function type?
  3. How do function types participate in ordinary Natcode type algebra and refinement operations, including union, intersection, difference, complement, optionality, and `where`?
  4. Is there a structural way to denote the set of all callable function values, or does the eventual design require a built-in `Function` type?
- **Notes:** Function-type syntax is PAR-FN-06. This item does not decide the general assignability model in TYP-CON-03 or add a built-in `Function`; it records the function-specific questions that must integrate with those designs.


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

### TYP-EVAL-04 — Unknown mandatory restrictions

- **Status:** Approved
- **Spec:** [evaluation.md — Function return inference: context-independent](../spec/evaluation.md#function-return-inference-context-independent)
- **Done when:** A mandatory `where` restriction proven false is a compile error, and a restriction the compiler cannot establish is also rejected rather than treated as true or false.
- **Notes:** Exact evaluation budgets and the full compile-time function eligibility boundary remain TYP-EVAL-02; explicit runtime validation remains TYP-REF-12.

### TYP-EVAL-05 — Flow-sensitive symbolic reasoning

- **Status:** Approved
- **Spec:** [evaluation.md — Flow-sensitive refinement and symbolic reasoning](../spec/evaluation.md#flow-sensitive-refinement-and-symbolic-reasoning)
- **Done when:** The type checker carries established true and false facts through control flow and uses supported logical implication to establish restricted-type membership; the guaranteed solver boundary is TYP-REF-10.

### TYP-DIAG-01 — Compile errors versus warnings

- **Status:** Approved
- **Spec:** [evaluation.md — Function return inference: context-independent](../spec/evaluation.md#function-return-inference-context-independent)
- **Done when:** Missing or disproven mandatory guarantees cause errors; known redundancy in otherwise valid code can warn, except provably unreachable `when` branches (including redundant `_`) which are compilation errors under TYP-PAT-01; optional analysis failure alone has no diagnostic.

### TYP-DIAG-02 — Plain-language actionable diagnostics

- **Status:** Approved
- **Spec:** [evaluation.md — Diagnostics](../spec/evaluation.md#diagnostics)
- **Done when:** Compile errors are understandable without compiler/type-theory jargon and, when possible, explain what was written, why Natcode cannot accept it, and a concrete form that would work instead.

### TYP-DIAG-03 — Diagnostics for statically decided runtime validation

- **Status:** Open question
- **Spec:** [evaluation.md — Diagnostics](../spec/evaluation.md#diagnostics)
- **Questions:**
  1. Should a valid `as?` application warn or otherwise report when the compiler can already prove that it must succeed?
  2. Should it warn or otherwise report when the compiler can already prove that it must fail?
- **Notes:** Regardless of diagnostics, TYP-REF-13 keeps the stable `T | ValidationError` signature and permits compile-time elimination of runtime work.

### TYP-DIAG-04 — Compiler diagnostic mode configuration

- **Status:** Open question
- **Spec:** [evaluation.md — Diagnostics](../spec/evaluation.md#diagnostics)
- **Questions:**
  1. How is strict versus non-strict compiler mode selected, and which mode is the default?
  2. Which otherwise valid constructs, beyond the now-approved redundant `??` diagnostic in TYP-DIAG-06, warrant a diagnostic?
  3. How should non-strict mode classify new diagnostics while preserving existing unconditional errors, including unreachable `when` branches (TYP-DIAG-01 and TYP-PAT-01)?
  4. Does mode selection affect only diagnostic severity, without changing type inference or runtime semantics?
- **Notes:** The existence of strict and non-strict modes and strict mode's error-only diagnostic behavior are approved under TYP-DIAG-05. This item tracks only the remaining unresolved design questions.

### TYP-DIAG-06 — Redundant coalescing is diagnosed at its definition

- **Status:** Approved
- **Spec:** [evaluation.md — Optional coalescing evaluation](../spec/evaluation.md#optional-coalescing-evaluation)
- **Done when:** A `??` whose left operand provably excludes `none` is diagnosed at that expression: warning in non-strict mode, compile error in strict mode. Diagnostics are based on the containing function's permitted input domain, never on a particular call site's specialization. A certainly-`none` left operand is not a redundancy diagnostic.

### TYP-DIAG-05 — Strict mode reports problematic code as errors

- **Status:** Approved
- **Spec:** [evaluation.md — Diagnostics](../spec/evaluation.md#diagnostics)
- **Done when:** The compiler supports strict and non-strict diagnostic modes. In strict mode, every diagnosed problematic construct is a compile error, never a warning, including cases that would only produce warnings in non-strict mode. Mandatory errors remain errors in both modes; inconclusive optional analyses do not acquire diagnostics just because strict mode is enabled. Which constructs warrant diagnostics and how modes are selected remain TYP-DIAG-04.

### TYP-PAT-01 — Exhaustiveness and redundant catch-all

- **Status:** Approved
- **Spec:** [syntax-and-types.md — When expressions](../spec/syntax-and-types.md#when-expressions)
- **Done when:** Subject-based and subjectless `when` are exhaustive: remaining possibilities must be provably `Empty` or a final `_` branch must cover them. `_` represents the remaining `Any` case, occurs at most once and only as the final branch, and is a compile error if redundant. Every provably unreachable branch is a compile error (not a warning). Constructors eliminated by a restricted subject type need no branch, and arbitrary predicates alone do not prove exhaustiveness.

## Optional values, coalescing and record types

### TYP-OPT-01 — Transparent nullable postfix types

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Optional types and absence coalescing](../spec/syntax-and-types.md#optional-types-and-absence-coalescing)
- **Done when:** `T?` denotes `T | None` for every type expression, with ordinary exact set normalization; `T?? = T?`, `Empty? = None`, and `Any? = Any`. A type constraint does not widen exact literal inference.

### TYP-OPT-02 — Optional record-key presence and safe access

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Type-expression computations and record syntax](../spec/syntax-and-types.md#type-expression-computations-and-record-syntax)
- **Done when:** `{ age?: Number }` permits absent `age` or a present numeric value, distinct from required `{ age: Number? }`; direct access to a possibly missing key errors until its presence is established. Missing and present `none` remain distinct. Width subtyping and intersections preserve absence possibilities and record-shape correlations.

### TYP-OPT-03 — Exact coalescing result types and static checking

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Optional types and absence coalescing](../spec/syntax-and-types.md#optional-types-and-absence-coalescing)
- **Done when:** `A ?? B` infers `A` if non-`None`, `B` if definitely `None`, otherwise `(A \ None) | B`, simplified by provable subset relations, including `Any \ None`. Both operands are always checked in the full function input domain even if right is unreachable at a particular call site. Exact call-site return inference remains supported.

### TYP-OPT-04 — Record pattern membership, narrowing and exhaustiveness

- **Status:** Approved
- **Spec:** [syntax-and-types.md — When expressions](../spec/syntax-and-types.md#when-expressions)
- **Done when:** Record patterns test key presence, typed or literal values, and nested patterns; `{age}` captures while `{age: Number}` only tests and narrows; explicit `as` aliases capture. Earlier `when` matches narrow later branches by set difference, with provable unreachable branches rejected. `where` shares patterns while retaining its disjointness and no-fallthrough rules.

### TYP-OPT-05 — Nullable parameters do not imply optional arguments

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Function parameters and labeled calls](../spec/syntax-and-types.md#function-parameters-and-labeled-calls)
- **Done when:** A `T?` function parameter accepts `none` but remains a required argument unless an explicit default is declared, for example `age: Number? = none`.

## Arrays

### TYP-ARR-01 — Array type model and inference

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Code blocks and data structures](../spec/syntax-and-types.md#code-blocks-and-data-structures)
- **Questions:**
  1. What does Natcode mean by an array type, and is it distinct from any future list, tuple, or other sequence abstraction?
  2. How are array literal types inferred, including empty arrays, heterogeneous elements, exact per-position information, and element-type widening?
  3. Are fixed-length heterogeneous tuple-like values represented by array types or by a distinct type form?
  4. What are the assignability and variance rules for array types, and how do arrays participate in union, intersection, difference, complement, optionality, and `where`?
  5. Does the final array model require a compiler-provided public built-in `Array` type, or can its observable type semantics be expressed without one?
- **Notes:** Surface syntax is PAR-ARR-01 and runtime behavior is RUN-ARR-01. This item intentionally leaves the built-in inventory question unresolved until the array model is designed.

## Object types

### TYP-OBJ-01 — Dynamic object key typing

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Code blocks and data structures](../spec/syntax-and-types.md#code-blocks-and-data-structures)
- **Questions:**
  1. How closely are object types related to `Map`, if at all?
  2. How should the type system represent an object literal containing a computed key whose runtime value is not statically known?
  3. When a computed key may collide with a statically known property, how is the final property type inferred while preserving precise access to known properties and the eventual approved runtime collision semantics (RUN-OBJ-01)?
  4. When a computed key is known exactly, or is provably disjoint from a static property, how is that knowledge reflected in the resulting object type?
- **Notes:** Computed-key syntax itself is approved by PAR-OBJ-01. The representation and runtime collision/overwrite order (RUN-OBJ-01) are intentionally unresolved. For example, with `key: String`, `{ name: "Ada", [key]: 1234 }` must remain type-safe even though `key` may be `"name"`; a future design may relate the dynamic portion to `Map`, use intersections, or choose another representation without committing to any of those here.

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

### TYP-EXP-06 — Imports require guaranteed export-key presence

- **Status:** Approved
- **Spec:** [modules.md — Exported object shapes and optional properties](../spec/modules.md#exported-object-shapes-and-optional-properties)
- **Done when:** A named import is valid only if its key exists in every possible exported object shape; conditional absence causes an error, not an implicit `None` value. Explicit `none` in a present property is permitted.
- **Notes:** Reverses the former approval allowing imports of keys present in only some shapes. Wildcard import filtering is RES-IMP-03 and ordinary record presence is TYP-OPT-02.

## Open type-system subjects

### TYP-ERR-01 — Error designation and propagation details

- **Status:** Open question
- **Spec:** [evaluation.md — Error handling](../spec/evaluation.md#error-handling)
- **Questions:**
  1. How does a type receive the error designation (declaration syntax, marker type, protocol, or another mechanism)?
  2. How are possible errors reflected in type inference, union types, and function return types?
  3. How do callers handle, transform, and propagate error values without exceptions?
- **Notes:** Explicit, unwrapped error values and compiler-recognized designated error types are approved in TYP-ERR-02; automatic propagation of designated error operands through otherwise valid built-in arithmetic is approved in TYP-ERR-03. Language-level exceptions are rejected (RUN-EFF-04). **Defer remaining error designation, evaluation order, multiple-error behavior, general operator coverage, typing, and propagation details until the foundational type model is worked through**, especially TYP-ADT-02, TYP-CON-03, and PAR-TYPE-02. Do not assume marker types, structural inheritance, or nominal branding.

### TYP-ERR-02 — Designated first-class error types

- **Status:** Approved
- **Spec:** [evaluation.md — Error handling](../spec/evaluation.md#error-handling)
- **Done when:** Error values are ordinary first-class values; success values and designated error values may be returned directly as alternatives in a union, without obligatory `Ok`/`Err` wrappers. The type checker recognizes designated error types as distinct from ordinary non-error types.
- **Notes:** The mechanism of designating an error type, exact return-type inference, and propagation rules remain open in TYP-ERR-01.

### TYP-ERR-03 — Automatic propagation through arithmetic operators

- **Status:** Approved
- **Spec:** [evaluation.md — Error handling](../spec/evaluation.md#error-handling)
- **Done when:** For an otherwise valid built-in arithmetic operation with a designated error value as an operand, the original error is propagated instead of performing arithmetic on it; possible error outcomes remain in the expression's result type. For example, `Number | ValidationError` combined with `Number` may produce `Number | ValidationError` rather than a mandatory wrapper.
- **Notes:** Extending this principle to other applicable built-in operators is approved in principle, but exact coverage, evaluation order, short-circuiting, precedence among multiple errors, and precise typing remain open in TYP-ERR-01. This item does not determine how error types are designated. It also does not override the statically provable nonzero divisor requirement for successful division/modulo paths (TYP-ARITH-04).

### TYP-NONE-01 — Built-in `none` singleton and set membership

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Optional types and absence coalescing](../spec/syntax-and-types.md#optional-types-and-absence-coalescing)
- **Done when:** Hard-reserved literal `none` has exact singleton type `None = {none}`, which participates in ordinary type union/intersection/difference and is distinct from `Empty` and nominal `Option.None`; functions or operations with no other meaningful result, such as `print`, return `none`.
- **Notes:** Replaces proposed `Unit`, `null`/`Null`, and implicit absence-as-missing-value designs.

### TYP-ADT-01 — Nominal algebraic types with constructors

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** `unique type Option` with indented `Some(Any)` and `None` introduces a nominal sum-type and constructors accessible as `Option.Some` and `Option.None`; bare unqualified constructors are not introduced automatically.

### TYP-ADT-02 — Remaining algebraic type semantics

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Questions:**
  1. What further rules govern generic, recursive, nested and other advanced constructor definitions?
  2. What additional variant narrowing rules apply beyond the approved general `when` exhaustiveness requirement?

### TYP-GEN-01 — Explicit generics and advanced polymorphism

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Type-expression computations and record syntax](../spec/syntax-and-types.md#type-expression-computations-and-record-syntax)
- **Questions:**
  1. After implementing implicitly polymorphic function parameters (TYP-FN-06) and specialized constructor restrictions, which remaining examples, if any, genuinely require explicit generic type or function declarations, higher-order type functions, or type-level quantification syntax?
  2. If needed, how should those advanced cases be expressed and inferred?
- **Notes:** Explicit `<T>` is not approved. Ordinary function polymorphism is already approved independently of this question.

### TYP-NS-01 — Type values inside namespaces

- **Status:** Open question
- **Spec:** [modules.md — First-class namespaces and persistent updates](../spec/modules.md#first-class-namespaces-and-persistent-updates)
- **Questions:**
  1. How are types represented at runtime when they are members of a first-class namespace value?

## Built-in arithmetic operators

### TYP-ARITH-01 — True numeric division

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Numeric arithmetic operators](../spec/syntax-and-types.md#numeric-arithmetic-operators)
- **Done when:** Value-expression `/` computes true division, rather than truncating the quotient: `5 / 2` evaluates to `2.5` and `6 / 2` evaluates to `3`, subject to the required nonzero divisor proof (TYP-ARITH-04).
- **Notes:** Numeric representation, precision/rounding, and static result types involving `Number` and `Integer` remain open in TYP-BLT-05 and TYP-BLT-06.

### TYP-ARITH-02 — Truncated remainder for numeric and fractional operands

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Numeric arithmetic operators](../spec/syntax-and-types.md#numeric-arithmetic-operators)
- **Done when:** `%` computes the remainder associated with quotient truncation toward zero for finite numeric operands, including fractional operands: `-5 % 3` is `-2`, `5 % -3` is `2`, `5.5 % 2` is `1.5`, and `5 % 2.5` is `0`. The divisor must be provably nonzero (TYP-ARITH-04).
- **Notes:** This is the JavaScript/TypeScript remainder convention, not Euclidean modulo. Edge cases involving numeric representation, rounding, and non-finite values remain open in TYP-BLT-05/06.

### TYP-ARITH-03 — Euclidean modulo for numeric and fractional operands

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Numeric arithmetic operators](../spec/syntax-and-types.md#numeric-arithmetic-operators)
- **Done when:** A second built-in infix arithmetic operator computes Euclidean modulo on finite numeric operands, including fractional operands. For any nonzero divisor `b`, the result `r` satisfies `0 <= r < abs(b)`; examples include `-5.5 MOD 2 == 0.5` and `5.5 MOD -2 == 1.5`. The operation is left-associative, at the `*`/`/`/`%` precedence tier, and requires a provably nonzero divisor.
- **Notes:** `MOD` is a placeholder in examples, **not approved source syntax**. The actual token spelling is an open parser decision in PAR-EXPR-03. Representation-specific precision/rounding details remain open in TYP-BLT-05/06.

### TYP-ARITH-04 — Divisors must be provably nonzero

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Numeric arithmetic operators](../spec/syntax-and-types.md#numeric-arithmetic-operators)
- **Done when:** Numeric `/`, `%`, and Euclidean modulo require the divisor to be provably nonzero for the successful arithmetic path. Statically zero divisors and unknown divisors without established nonzero evidence cannot be used in arithmetic. A known nonzero literal or an established `where` refinement can supply the proof; explicit runtime validation with `as?` can establish the successful nonzero alternative and propagate its `ValidationError` alternative through arithmetic as approved in TYP-ERR-03.
- **Notes:** Runtime-validation result signatures remain stable unions under TYP-REF-13; details of error-aware evaluation and typing remain TYP-ERR-01. Exact numeric proof limits remain TYP-REF-10 and TYP-BRF-05/06.

## Built-in value types

Each built-in type is an independent scorecard item. Each type also has a distinct refinement item, even when it currently inherits only the general `where` semantics of TYP-REF-01 through TYP-REF-11. A refinement item does not imply new syntax, conversion behavior, or a dedicated constructor.

### TYP-BLT-01 — `Any` built-in

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Union and intersection types](../spec/syntax-and-types.md#union-and-intersection-types)
- **Done when:** `Any` denotes the universal type as established by TYP-UNI-07.
- **Notes:** Existing type-lattice rule; tracked independently here as a built-in.

### TYP-BLT-02 — `Empty` built-in

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Union and intersection types](../spec/syntax-and-types.md#union-and-intersection-types)
- **Done when:** `Empty` denotes a type with no inhabitants as established by TYP-UNI-07.
- **Notes:** Function termination, successful return guarantees, and whether functions may declare `Empty` returns are outside built-in type scope and remain open in the function semantics work.

### TYP-BLT-03 — `Unit` built-in (retired)

- **Status:** Superseded
- **Spec:** [syntax-and-types.md — Optional types and absence coalescing](../spec/syntax-and-types.md#optional-types-and-absence-coalescing)
- **Superseded by:** TYP-BLT-09
- **Notes:** The former singleton `Unit`/`()` design was replaced by the built-in literal `none` and singleton type `None`. The historical decision is retained; `Unit` is not a current built-in.

### TYP-BLT-04 — `Boolean` built-in

- **Status:** Approved
- **Spec:** [variables.md — Exact type inference](../spec/variables.md#exact-type-inference)
- **Done when:** `Boolean` contains exactly `true` and `false`; both have their exact literal types, assignable to `Boolean`.
- **Notes:** No separate Boolean wrapper type.

### TYP-BLT-05 — `Number` built-in semantics

- **Status:** Open question
- **Spec:** [variables.md — Exact type inference](../spec/variables.md#exact-type-inference)
- **Questions:**
  1. Is `Number` a concrete representation or a general numeric supertype, and how does it relate to `Integer`?
  2. What arithmetic representation, exactness, numeric literal classification, and rounding rules apply?
  3. Does `Number` include non-finite values such as NaN or infinities?
  4. Are precision/scale-aware numeric types expressible with the existing type system, or is additional capability needed?
- **Notes:** The language owner approved having `Number` as a built-in. True `/`, truncated `%` (including fractional operands), and Euclidean modulo (including fractional operands) are separately approved in TYP-ARITH-01 through TYP-ARITH-04. This item's representation, rounding, and precision questions remain open. `Float`, `Decimal`, and parameterized numeric precision are not approved as built-ins.

### TYP-BLT-06 — `Integer` built-in semantics

- **Status:** Open question
- **Spec:** [variables.md — Exact type inference](../spec/variables.md#exact-type-inference)
- **Questions:**
  1. Are integers arbitrary-precision or fixed-width?
  2. How does `Integer` relate to `Number` (subtyping, numeric family, or distinct representation)?
  3. What are overflow, coercion, and static result-type rules for mixed `Integer`/`Number` arithmetic and true division?
- **Notes:** The language owner approved having `Integer` as a built-in. Division is true rather than truncating (`5 / 2` gives `2.5`); numeric representation, static result types, overflow, and coercion remain open. Remainder/modulo semantics are separately approved in TYP-ARITH-02/03.

### TYP-BLT-07 — `String` built-in

- **Status:** Approved
- **Spec:** [variables.md — Exact type inference](../spec/variables.md#exact-type-inference)
- **Done when:** `String` is a built-in Unicode text type; exact string literals retain literal types. Default character-oriented string operations use Unicode grapheme clusters rather than Unicode code-point counts.
- **Notes:** Precise Unicode versioning, indexing API and slicing API remain string-library/operation design details, not prerequisites to the built-in decision.

### TYP-BLT-08 — No built-in `Char`

- **Status:** Approved
- **Spec:** [variables.md — Exact type inference](../spec/variables.md#exact-type-inference)
- **Done when:** The built-in type inventory does not contain a `Char` type; a single user-perceived character is representable as a `String`.
- **Notes:** Explicit rejection of `Char` as a built-in, not a deferred decision. A user-defined refinement alias does not make it a built-in.

### TYP-BLT-09 — `None` built-in

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Optional types and absence coalescing](../spec/syntax-and-types.md#optional-types-and-absence-coalescing)
- **Done when:** The built-in `None` contains exactly `none`, is distinct from `Empty`, replaces `Unit`, and is available without an import, as described by TYP-NONE-01.

### TYP-BLT-10 — Remaining built-in type inventory

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Built-in names and explicit qualification](../spec/syntax-and-types.md#built-in-names-and-explicit-qualification)
- **Done when:** A public Natcode built-in type exists only when its language semantics cannot be faithfully expressed as an ordinary Natcode-defined type; compiler/runtime implementation primitives do not automatically become public built-ins. The initial public built-in inventory is closed to the individually tracked built-ins, and any later addition receives its own built-in and refinement scorecard items.
- **Notes:** This closes the catch-all inventory question rather than leaving it permanently open. `ValidationError` is added by TYP-BLT-11 because the language-level `as?` operator intrinsically produces it. No built-in `Object` or `Record` is needed because `{}` is the structural record type with zero required properties and therefore accepts any record under width subtyping. No built-in `Function` or `Array` is introduced now; first-class function design is tracked by PAR-FN-06/TYP-FN-07 and array design by PAR-ARR-01/TYP-ARR-01/RUN-ARR-01. Built-in status for `Map`, a public `Type`/metatype, `Byte`/`Bytes`, `Set`, `List`/sequence, and tuple-like concepts is deferred until those abstractions are designed. No general built-in `Error` is introduced at this stage; arbitrary error designation remains TYP-ERR-01.

### TYP-BLT-11 — `ValidationError` built-in

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Runtime validation](../spec/syntax-and-types.md#runtime-validation)
- **Done when:** `ValidationError` is a public built-in designated error type, available without an import and explicitly addressable as `@ValidationError`, because the language-level `as?` operator intrinsically produces it; every valid `x as? T` has result type `T | ValidationError` as established by TYP-REF-13.
- **Notes:** The exact data/constructor shape, validation messages, metadata, and nested failure representation remain open in TYP-REF-16. This item does not introduce a general built-in `Error` type.

## Built-in refinements: individual items

### TYP-BRF-01 — `Any where ...`

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Refinement types with `where`](../spec/syntax-and-types.md#refinement-types-with-where)
- **Done when:** A refinement over `Any` follows the ordinary transparent-subset and proof rules, without introducing a new nominal identity.

### TYP-BRF-02 — `Empty where ...`

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Where patterns and filtering](../spec/syntax-and-types.md#where-patterns-and-filtering)
- **Done when:** A refinement over `Empty` remains `Empty`, following TYP-REF-07.

### TYP-BRF-03 — `Unit where ...` (retired)

- **Status:** Superseded
- **Spec:** [syntax-and-types.md — Optional types and absence coalescing](../spec/syntax-and-types.md#optional-types-and-absence-coalescing)
- **Superseded by:** TYP-BRF-09
- **Notes:** Refinement of the retired `Unit` built-in is replaced by refinement of the `None` singleton.

### TYP-BRF-09 — `None where ...`

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Optional types and absence coalescing](../spec/syntax-and-types.md#optional-types-and-absence-coalescing)
- **Done when:** `None where ...` follows normal transparent `where` filtering: it selects `none` or resolves to `Empty` according to its Boolean predicate, without introducing a new nominal identity.

### TYP-BRF-04 — `Boolean where ...`

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Refinement types with `where`](../spec/syntax-and-types.md#refinement-types-with-where)
- **Done when:** A `Boolean` refinement follows the general `where` rules and can select either, both, or neither Boolean value without introducing nominal identity.

### TYP-BRF-05 — `Number where ...`

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Refinement types with `where`](../spec/syntax-and-types.md#refinement-types-with-where)
- **Questions:**
  1. Once the `Number` representation and arithmetic semantics are chosen, which numeric predicates have guaranteed decidable/provable membership?
  2. How do precision/scale constraints interact with `Number where ...`, and what cannot be represented by refinements alone?
- **Notes:** General `where` semantics are already approved by TYP-REF-01 through TYP-REF-11; this item tracks only numeric-specific semantics.

### TYP-BRF-06 — `Integer where ...`

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Refinement types with `where`](../spec/syntax-and-types.md#refinement-types-with-where)
- **Questions:**
  1. Given the chosen integer representation, which integer-specific predicates and ranges have guaranteed compile-time proof?
  2. How are integer-only restrictions interpreted relative to `Number` and its refinements?
- **Notes:** General `where` behavior is not reopened. This is independent of TYP-BRF-05.

### TYP-BRF-07 — `String where ...`

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Refinement types with `where`](../spec/syntax-and-types.md#refinement-types-with-where)
- **Questions:**
  1. Which string predicates, including grapheme-cluster length, have guaranteed compile-time support?
  2. How are Unicode normalization and Unicode-version-dependent predicates treated in string refinements?
- **Notes:** The `String` built-in and grapheme-based default operations are approved independently in TYP-BLT-07. String refinement capability was previously left for future design.

### TYP-BRF-11 — `ValidationError where ...`

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Refinement types with `where`](../spec/syntax-and-types.md#refinement-types-with-where)
- **Done when:** A `ValidationError where ...` refinement follows the ordinary transparent-subset rules and introduces no new nominal identity; which internal fields, constructor shapes, or failure metadata can be inspected remains governed by TYP-REF-16.
- **Notes:** This item records the required per-built-in refinement behavior without deciding the still-open payload representation of `ValidationError`.
