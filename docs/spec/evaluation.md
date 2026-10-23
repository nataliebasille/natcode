# Type inference, lazy evaluation, and debugging

Item status and open questions for this file are tracked in the [types](../scorecards/types.md), [runtime](../scorecards/runtime.md), and [debugger](../scorecards/debugger.md) scorecards.

## Literal precision

- Literal expressions have exact literal types by default.
- Supported core constant operations infer exact types deterministically: `40 + 2` infers `42`. This is a static typing rule, not the result of optional optimization.
- Type constraints (`:`) check assignability without widening.

## Function return inference: context-independent

The designer revised an earlier broader constant-evaluation proposal: **function return types are inferred from parameter types and the function's own body**, not specialized using the specific values passed at each call site, nor values outside the function. Exact types can propagate through function-local variables.

```natcode
// Illustration of intended syntax and inference:
test(x: number) => x * 2   // return: number
test(x: 24) => x * 2       // return: 48
```

The function syntax in these examples is illustrative (PAR-FN-01, TYP-FN-03); they illustrate type inference rules only.

Optional compile-time evaluation must not change inferred types, runtime semantics, or whether a program is valid (TYP-EVAL-01). Pure functions with statically known inputs may be evaluated at compile time to validate mandatory `where` restrictions (TYP-EVAL-03). A restriction proven false is a compile error, and required validation cannot silently fall back to a broader type. If the compiler cannot establish a mandatory restriction, that is also a compile error; unknown is not treated as either true or false (TYP-EVAL-04). Exact evaluation budgets and the complete eligibility rules for compile-time function evaluation remain open (TYP-EVAL-02).

## Flow-sensitive refinement and symbolic reasoning

Natcode carries facts established by control flow into type checking (TYP-EVAL-05). A successful Boolean condition refines the bindings used by that condition inside its branch, while later branches also learn from the conditions that previously failed.

For example, with:

```natcode
type Positive = number where
    x => x > 0
```

inside a branch guarded by `x > 0`, `x` is known to satisfy `Positive`. If a previous branch tested `x > 0` and did not run, the remaining path knows that condition is false. With ordered numeric values, after excluding both `x > 0` and `x < 0`, the remaining path can refine `x` to `0`.

Refinement facts compose through Boolean conjunction, disjunction, and negation. Pure named predicates also contribute facts: when a restriction is defined by a pure predicate, a branch in which that same predicate is known true may establish membership in the restricted type. Destructuring a restricted constructor preserves the facts that apply to its extracted parameters (TYP-REF-08).

The initial symbolic solver must guarantee a practical decidable subset of numeric reasoning (TYP-REF-10):

- equality and ordered comparisons between values;
- Boolean combinations of those relations;
- transitive implications such as `x > y` and `y > 0` implying `x > 0`;
- linear arithmetic with known constant coefficients, such as `x + 5 > y`, `x - y <= 10`, and `2 * x > y`.

Predicates outside that guaranteed symbolic subset remain valid. Multiplication or division between two unknown values, powers, and other nonlinear relationships are not required to be proven symbolically in the initial solver. When all relevant inputs are statically known, ordinary pure compile-time evaluation may still validate such a predicate directly. Separately, successful `/`, `%`, and Euclidean modulo arithmetic require proof that the divisor is nonzero; an unknown divisor without such a proof is rejected (TYP-ARITH-04). This is a required operand guarantee, not a claim that the full quotient or remainder must be symbolically computed.

A failure to prove a fact outside the supported solver is **not** proof that the fact is false. It simply cannot satisfy a mandatory static restriction unless another known fact establishes it. When the source value is already statically compatible with a restricted target's validation input type, `as?` may explicitly perform the remaining membership check as runtime validation (TYP-REF-12). Its result type remains `Target | ValidationError` even if compile-time evaluation or symbolic reasoning can prove the particular application will succeed or fail; such proof may eliminate runtime work but does not alter the validator's stable signature (TYP-REF-13).

## Diagnostics

Violated or unestablished mandatory guarantees are compile errors; provably redundant but valid code may warn; unsuccessful optional analyses need no diagnostic. Warnings do not prevent compilation or change semantics (TYP-DIAG-01).

Natcode diagnostics should be understandable without compiler or type-theory terminology (TYP-DIAG-02). When possible, an error should explain in ordinary language what the developer wrote, why Natcode cannot accept it, and what form would work instead. Invalid `where` forms are hard compile errors with source spans, and a concrete rewrite should be shown when the compiler can confidently provide one. Whether a valid `as?` whose success or failure is already statically known should produce an informational or warning diagnostic remains open (TYP-DIAG-03).

## Error handling

Natcode has no language-level exceptions, throwing/catching semantics, or exception-based propagation (RUN-EFF-04). Recoverable errors are represented explicitly as values. `try`, `catch`, and `throw` are not built-in exception constructs (LEX-KW-11).

Error values are ordinary first-class values, but their types are designated as errors and recognized by the type checker (TYP-ERR-02). Operations may produce a success value or an error value directly as alternatives in a union, without mandatory `Ok` and `Err` wrappers. For example, `number | ParseError` can represent either a successful number or a designated parse error (how error types are designated remains open).

`ValidationError` is the designated error alternative produced by explicit restricted-type validation with `as?`; it captures type-level validation failures while preserving the requested target as the success alternative (TYP-REF-12, TYP-REF-15). Its exact payload/message representation remains open (TYP-REF-16).

Built-in arithmetic operators automatically propagate designated error operands rather than attempting arithmetic on them (TYP-ERR-03); the broader principle of propagating errors through other suitable operators is approved in principle. Exact error designation, type-inference mechanics, evaluation order, multiple-error handling, and other operator coverage remain open (TYP-ERR-01). Unrecoverable runtime faults and failures in lazy computations remain a separate question (RUN-EFF-02).

## Laziness

- Top-level module bindings and exported object members are call-by-need and cached after evaluation.
- Unused work should not execute, regardless of whether the enclosing object originates in an expression or factory function, subject to actual data dependencies.
- Persistent immutable data structures allow versions to share storage; memoization avoids repeated computation. These are complementary, not synonymous.

## Debugging

Debug builds **preserve the language's lazy semantics**. Debugging tools may show suspended computations and let the developer explicitly force/step into them. Automatically evaluating every function in debug mode is rejected, because it changes program behavior. Production builds may remove unused pure computations and debug builds may retain richer source metadata.

The debugger interface, effects, resource limits, and force semantics are DBG-API-01.
