# Types scorecard

- **Prefix:** TYP
- **Covers:** Nominal types, constructors and aliases; type inference and checking; set-theoretic union and intersection; function return inference; compile-time evaluation; module export kinds and shapes.

## Nominal types and aliases

### TYP-NOM-01 — Nominal type identity

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** `unique type UserId(number)` and `unique type ProductId(number)` introduce distinct identities; values of one are not interchangeable with the other merely because both wrap `number`.

### TYP-NOM-02 — Strict nominal payload boundary

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** A nominal value does not implicitly convert to its payload type or inherit the payload's operators; operator definitions must be explicit when designed.

### TYP-NOM-03 — Multiple positional constructor parameters

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** `unique type Point(number, number)` allows `Point(10, 20)` and each parameter is typed independently.

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
- **Done when:** `unique type Sprint(Position, Color)` takes constructed `Position` and `Color` values as ordinary arguments; writing `Position(number, number)` or `Color(string)` where these do not restrict the declared type has the same parameter meaning.
- **Notes:** No special nominal-combination operator, multiple inheritance, interfaces, classes, or automatic nested-value projection is approved.

### TYP-NOM-07 — Specialized constructor aliases expose remaining parameters

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Specialized aliases and constructors](../spec/syntax-and-types.md#specialized-aliases-and-constructors)
- **Done when:** `type XAxis = Position(number, 0)` exposes one constructor parameter and `XAxis(10)` constructs `Position(10, 0)`; `Position(0, 0)` exposes zero parameters.

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

### TYP-UNI-05 — Complement types

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Union and intersection types](../spec/syntax-and-types.md#union-and-intersection-types)
- **Questions:**
  1. Is there a standalone complement type operator, and with what syntax?
- **Notes:** Binary set difference is approved separately as `/` in TYP-UNI-10.

### TYP-UNI-10 — Set difference with `/`

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Union and intersection types](../spec/syntax-and-types.md#union-and-intersection-types)
- **Done when:** In type expressions, `A / B` denotes exactly the values in `A` that are not in `B`; normalization may preserve constructibility when the result still has one effective constructor shape.

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

### TYP-PAT-01 — Exhaustiveness and redundant catch-all

- **Status:** Approved
- **Spec:** [syntax-and-types.md — When expressions](../spec/syntax-and-types.md#when-expressions)
- **Done when:** Subject-based and subjectless `when` are exhaustive: remaining possibilities must be provably `Empty` or a final `_` branch must cover them. `_` represents the remaining `Any` case, occurs at most once and only as the final branch, and is a compile error if redundant. Every provably unreachable branch is a compile error (not a warning). Constructors eliminated by a restricted subject type need no branch, and arbitrary predicates alone do not prove exhaustiveness.

## Object types

### TYP-OBJ-01 — Dynamic object key typing

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Code blocks and data structures](../spec/syntax-and-types.md#code-blocks-and-data-structures)
- **Questions:**
  1. How closely are object types related to `Map`, if at all?
  2. How should the type system represent an object literal containing a computed key whose runtime value is not statically known?
  3. When a computed key may collide with a statically known property, how is the final property type inferred while preserving source-order overwrite behavior and precise access to known properties?
  4. When a computed key is known exactly, or is provably disjoint from a static property, how is that knowledge reflected in the resulting object type?
- **Notes:** Computed-key syntax itself is approved by PAR-OBJ-01. The representation is intentionally unresolved. For example, with `key: String`, `{ name: "Ada", [key]: 1234 }` must remain type-safe even though `key` may be `"name"`; a future design may relate the dynamic portion to `Map`, use intersections, or choose another representation without committing to any of those here.

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
- **Done when:** `unique type Option` with indented `Some(Any)` and `None` introduces a nominal sum-type and constructors accessible as `Option.Some` and `Option.None`; bare unqualified constructors are not introduced automatically.

### TYP-ADT-02 — Remaining algebraic type semantics

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Questions:**
  1. What further rules govern generic, recursive, nested and other advanced constructor definitions?
  2. What additional variant narrowing rules apply beyond the approved general `when` exhaustiveness requirement?

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
