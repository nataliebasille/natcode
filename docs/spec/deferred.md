# Deferred and not-yet-finalized subjects

These are **not approved rules** and should not be treated as completed specification text.

- **Entry functions:** argument(s), command-line parameters, runtime context, return contract, effects/purity, and external capability handling.
- **Function syntax and return inference:** complete grammar, overloading, detailed context-independent analysis, recursion, type precision, and standardized constant-evaluation budgets.
- **Types:** exact status/representation of `None`, assignability and subtype relations, algebraic type definitions, constructors, intersections/unions, runtime representation of type values within first-class namespaces.
- **General objects:** literal grammar, computed/dynamic keys, nested property updates such as `Math.PI = value`, persistence/laziness details beyond approved outcomes.
- **Imports:** final grammar of combinations such as named imports plus a wildcard remainder; package/built-in import specifiers; JSON module representation and other external data formats; exact handling of aliases or conflicts within a single compound import.
- **Extensions/constructors:** syntax for associating independently defined functions with types; selection, conflicts, visibility and import behavior.
- **Lexer / grammar:** keyword list; precise Unicode normalization and emoji-sequence admission policy; statement and block syntax; type and object grammars.
- **Build system:** package layout, CLI details, backend mapping to JavaScript/WebAssembly/C/Python, linking and source maps.
- **Lazy execution and effects:** impure computations, memoization across runtime scopes, dependency-driven forcing through arbitrary user-defined factories, errors/nontermination, debug force semantics.

The immediate design process remains: address **one unresolved question at a time**, keep decisions distinct from illustrative code, then request a final approval for each specification area.
