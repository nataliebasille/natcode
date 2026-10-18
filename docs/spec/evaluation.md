# Type inference, lazy evaluation, and debugging

Item status and open questions for this file are tracked in the [types](../scorecards/types.md), [runtime](../scorecards/runtime.md), and [debugger](../scorecards/debugger.md) scorecards.

## Literal precision

- Literal expressions have exact literal types by default.
- Pure constant operations may propagate exact results: `40 + 2` can infer `42`.
- Type constraints (`:`) check assignability without widening.

## Function return inference: context-independent

The designer revised an earlier broader constant-evaluation proposal: **function return types are inferred from parameter types and the function's own body**, not specialized using the specific values passed at each call site, nor values outside the function. Exact types can propagate through function-local variables.

```natcode
// Illustration of intended syntax and inference:
test(x: number) => x * 2   // return: number
test(x: 24) => x * 2       // return: 48
```

The function syntax in these examples is illustrative (PAR-FN-01, TYP-FN-03); they illustrate type inference rules only.

Optional compile-time evaluation must not change inferred types, runtime semantics, or whether a program is valid (TYP-EVAL-01). Pure functions with statically known inputs may be evaluated at compile time to validate type-level constraints (TYP-EVAL-03). False mandatory constraints are compile errors; required validation cannot silently fall back to a broader type. Eligible functions, resource limits, unknown inputs, and constraint syntax remain open (TYP-EVAL-02, TYP-EVAL-04). More general proof-based reasoning is deferred (TYP-EVAL-05).

Diagnostics (TYP-DIAG-01): violated or unestablished mandatory guarantees are errors; provably redundant but valid code may warn; unsuccessful optional analyses need no diagnostic. Warnings do not prevent compilation or change semantics.

## Error handling

Natcode has no language-level exceptions, throwing/catching semantics, or exception-based propagation (RUN-EFF-04). Recoverable errors are represented explicitly as values. `try`, `catch`, and `throw` are not built-in exception constructs (LEX-KW-11).

Error values are ordinary first-class values, but their types are designated as errors and recognized by the type checker (TYP-ERR-02). Operations may produce a success value or an error value directly as alternatives in a union, without mandatory `Ok` and `Err` wrappers. For example, `number | ParseError` can represent either a successful number or a designated parse error (how error types are designated remains open).

How types receive the error designation, how possible errors are inferred in signatures, and how errors are propagated and handled remain open (TYP-ERR-01). Unrecoverable runtime faults and failures in lazy computations remain a separate question (RUN-EFF-02).

## Laziness

- Top-level module bindings and exported object members are call-by-need and cached after evaluation.
- Unused work should not execute, regardless of whether the enclosing object originates in an expression or factory function, subject to actual data dependencies.
- Persistent immutable data structures allow versions to share storage; memoization avoids repeated computation. These are complementary, not synonymous.

## Debugging

Debug builds **preserve the language's lazy semantics**. Debugging tools may show suspended computations and let the developer explicitly force/step into them. Automatically evaluating every function in debug mode is rejected, because it changes program behavior. Production builds may remove unused pure computations and debug builds may retain richer source metadata.

The debugger interface, effects, resource limits, and force semantics are DBG-API-01.
