# Runtime scorecard

- **Prefix:** RUN
- **Covers:** The execution model shared by the interpreter and all backends: lazy evaluation, memoization, namespaces as values, persistent updates, entry functions, and effects. Per-target conformance is tracked in each execution scorecard.

## Laziness

### RUN-LAZY-01 — Demand-driven top-level bindings

- **Status:** Approved
- **Spec:** [modules.md — Lazy module evaluation](../spec/modules.md#lazy-module-evaluation)
- **Done when:** Importing a module does not evaluate its unused top-level initializers.

### RUN-LAZY-02 — Memoized bindings

- **Status:** Approved
- **Spec:** [evaluation.md — Laziness](../spec/evaluation.md#laziness)
- **Done when:** Repeatedly using a binding runs its initializer at most once and reuses the result.

### RUN-LAZY-03 — Independent lazy object members

- **Status:** Approved
- **Spec:** [modules.md — Lazy module evaluation](../spec/modules.md#lazy-module-evaluation)
- **Done when:** Accessing exported `cheap` does not force an unrelated `expensiveCalculation()` member.

### RUN-LAZY-04 — Laziness through object-producing functions

- **Status:** Approved
- **Spec:** [evaluation.md — Laziness](../spec/evaluation.md#laziness)
- **Done when:** A factory can return an object whose member computations stay suspended, and unused ones never run.

### RUN-LAZY-05 — Forcing through arbitrary function calls

- **Status:** Open question
- **Spec:** [modules.md — Lazy module evaluation](../spec/modules.md#lazy-module-evaluation)
- **Questions:**
  1. How is dependency-driven forcing lowered through arbitrary user-defined factory functions?
  2. Where is the purity/effect boundary for lazy members?

## Namespaces and updates

### RUN-NS-01 — First-class namespace values

- **Status:** Approved
- **Spec:** [modules.md — First-class namespaces and persistent updates](../spec/modules.md#first-class-namespaces-and-persistent-updates)
- **Done when:** An imported namespace can be assigned, passed to a function, returned, and nested in another object or exported namespace.

### RUN-NS-02 — Shared original namespace

- **Status:** Approved
- **Spec:** [modules.md — First-class namespaces and persistent updates](../spec/modules.md#first-class-namespaces-and-persistent-updates)
- **Done when:** Separate imports of one module refer to the same namespace value without initializing the module again.

### RUN-UPD-01 — Dotted updates create new versions

- **Status:** Approved
- **Spec:** [modules.md — First-class namespaces and persistent updates](../spec/modules.md#first-class-namespaces-and-persistent-updates)
- **Done when:** `Math.PI = value` produces a new `Math` value for the current binding; aliases and other imports keep the original.
- **Notes:** Needs update syntax (PAR-UPD-01).

### RUN-UPD-02 — General object update semantics

- **Status:** Open question
- **Spec:** [modules.md — First-class namespaces and persistent updates](../spec/modules.md#first-class-namespaces-and-persistent-updates)
- **Questions:**
  1. What are the semantics of nested property updates beyond the approved namespace outcome?
  2. How do updates interact with lazy, not-yet-evaluated members?

### RUN-UPD-03 — Structural sharing between versions

- **Status:** Proposed
- **Spec:** [evaluation.md — Laziness](../spec/evaluation.md#laziness)
- **Done when:** After an update, unchanged parts of the old and new values share storage.
- **Notes:** The spec names persistent data structures as an intended technique, not a rule.

### RUN-OBJ-01 — Computed and dynamic object keys

- **Status:** Open question
- **Spec:** [modules.md — Exported object shapes and optional properties](../spec/modules.md#exported-object-shapes-and-optional-properties)
- **Questions:**
  1. Can objects have computed or dynamic keys, and how are they accessed?
- **Notes:** Dynamic-key objects may be exported only inside a statically named property.

## Entry functions and effects

### RUN-ENTRY-01 — Entry function contract

- **Status:** Open question
- **Spec:** [modules.md — Entry modules](../spec/modules.md#entry-modules)
- **Questions:**
  1. What parameters does an entry function take (command-line arguments, runtime context)?
  2. What does it return, and how does that map to the process result?
  3. How are effects and external capabilities provided to it?

### RUN-EFF-01 — Effects and purity

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Inference-first philosophy](../spec/syntax-and-types.md#inference-first-philosophy)
- **Questions:**
  1. Purity is a design goal. How are impure computations expressed and tracked?
  2. How do effects interact with lazy evaluation?

### RUN-EFF-02 — Errors and nontermination in lazy computations

- **Status:** Open question
- **Spec:** [evaluation.md — Laziness](../spec/evaluation.md#laziness)
- **Questions:**
  1. When a forced computation fails or does not terminate, what happens, and is a failure memoized?

### RUN-EFF-03 — Memoization across runtime scopes

- **Status:** Open question
- **Spec:** [evaluation.md — Laziness](../spec/evaluation.md#laziness)
- **Questions:**
  1. Are memoized results shared across program runs, threads, or other runtime scopes?

### RUN-EFF-04 — No exceptions or exception propagation

- **Status:** Approved
- **Spec:** [evaluation.md — Error handling](../spec/evaluation.md#error-handling)
- **Done when:** Natcode provides no language-level exception mechanism, catching/throwing semantics, or exception-based propagation for recoverable errors. Such failures are represented explicitly as values.
- **Notes:** The error-value design is TYP-ERR-01. Runtime faults and failures of suspended computations remain an open subject (RUN-EFF-02); this decision does not prescribe fault termination or recovery behavior.
