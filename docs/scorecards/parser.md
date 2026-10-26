# Parser scorecard

- **Prefix:** PAR
- **Covers:** Turning tokens into syntax trees: module file structure, declarations, imports, exports, expressions, blocks, and syntax errors.

## Module file structure

### PAR-MOD-01 — Top-level declarations only

- **Status:** Approved
- **Spec:** [modules.md — What is a module?](../spec/modules.md#what-is-a-module)
- **Done when:** A file parses top-level bindings, imports, and an export; a standalone top-level expression such as a call is rejected.

### PAR-MOD-02 — Imports precede declarations

- **Status:** Approved
- **Spec:** [modules.md — Module form](../spec/modules.md#module-form)
- **Done when:** An import after an ordinary declaration is rejected.

### PAR-MOD-03 — Exactly one terminal export

- **Status:** Approved
- **Spec:** [modules.md — Module form](../spec/modules.md#module-form)
- **Done when:** `export {}` alone is a valid module; a missing export, any declaration after the export, and multiple exports are each rejected.

## Declarations

### PAR-DECL-01 — Keyword-free declarations

- **Status:** Approved
- **Spec:** [variables.md — Declaration syntax](../spec/variables.md#declaration-syntax)
- **Done when:** `x = 42` parses as a declaration without any `val`/`let`/`var` keyword.

### PAR-DECL-02 — Optional type constraint

- **Status:** Approved
- **Spec:** [variables.md — Declaration syntax](../spec/variables.md#declaration-syntax)
- **Done when:** `x: Number = 42` parses with the constraint attached to the declaration.

### PAR-DECL-03 — Initializer required

- **Status:** Approved
- **Spec:** [variables.md — Declaration syntax](../spec/variables.md#declaration-syntax)
- **Done when:** `x: Number` without `= expression` is rejected.

### PAR-DECL-04 — Underscore cannot be bound

- **Status:** Approved
- **Spec:** [variables.md — Identifier categories](../spec/variables.md#identifier-categories)
- **Done when:** `_ = 1` is rejected as a declaration.

## Imports

### PAR-IMP-01 — File-first import with mandatory clause

- **Status:** Approved
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** `from "./math" import …` parses; a bare `from "./math"` is rejected.

### PAR-IMP-02 — Static string-literal paths

- **Status:** Approved
- **Spec:** [modules.md — Paths and dependencies](../spec/modules.md#paths-and-dependencies)
- **Done when:** An identifier or other expression in place of the path string is rejected.

### PAR-IMP-03 — Namespace import

- **Status:** Approved
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** `from "./math" import Math` parses as a namespace import.

### PAR-IMP-04 — Named imports

- **Status:** Approved
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** `from "./math" import { PI, square }` parses as named imports.

### PAR-IMP-05 — Wildcard import

- **Status:** Approved
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** `from "./math" import *` parses as a wildcard import; `import _` is not accepted as a wildcard.

### PAR-IMP-06 — Combined named and namespace import

- **Status:** Approved
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** `from "./math" import { PI }, Math` parses as one combined import.

### PAR-IMP-07 — Import aliases

- **Status:** Approved
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Done when:** `from "./math" import { compute as calculate }` parses with the exported/original name first and new local name second; the old `{ calculate = compute }` alias spelling is invalid.

### PAR-IMP-08 — Combined wildcard and named import

- **Status:** Open question
- **Spec:** [modules.md — Import syntax and ordering](../spec/modules.md#import-syntax-and-ordering)
- **Questions:**
  1. What is the grammar for importing some names explicitly and the remaining names unqualified via wildcard (for example `*, { … }`)?

## Exports

### PAR-EXP-01 — Export object form

- **Status:** Approved
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** `export { PI, square }` and `export {}` parse as export objects.

### PAR-EXP-02 — Export aliases

- **Status:** Approved
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** `export { square as squareNumber }` parses with the original local name first and public alias second; the old `{ squareNumber = square }` alias spelling is invalid.

### PAR-EXP-03 — Export of an arbitrary expression

- **Status:** Approved
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** `export run`, `export api`, and `export makeApi()` parse as an export of an expression.

## Expressions, blocks, and other grammar

### PAR-BLK-01 — Braces delimit executable blocks

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Code blocks and data structures](../spec/syntax-and-types.md#code-blocks-and-data-structures)
- **Done when:** Named function bodies and multi-expression arrow/`when` branch bodies use `{ … }` as explicit executable blocks; the final expression supplies the block value without `return`. A single-expression arrow or branch body may omit braces. A block written with braces remains a block even when it contains only one expression.
- **Notes:** Replaces the earlier approved rule that `{}` could only denote data. Indentation does not establish arbitrary executable scopes. Immediately after `=>`, `{ … }` is a block; an anonymous object value uses `=> ({ key: value })` (PAR-OBJ-02).

### PAR-BLK-02 — Remaining statement and block grammar

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Code blocks and data structures](../spec/syntax-and-types.md#code-blocks-and-data-structures)
- **Done when:** Executable bodies parse newline-separated local bindings and expressions, with the final expression supplying the block result; two complete statements on one physical line and semicolons are rejected. An expression can continue across lines after a trailing binary operator requiring a right operand, or inside explicit parentheses; a leading operator alone never continues a completed preceding line, and backslash continuation is invalid. Continuations do not change `when`/`where`/constructor branch ownership, and ambiguous layouts are rejected. Standalone `{ ... }` executable block expressions are invalid; executable braces remain limited to named-function bodies and approved lambda/`when` branch bodies.
- **Notes:** Approved 2026-10-23. Preserves PAR-BLK-01 and PAR-FN-03, including one-expression braced blocks and implicit final-expression results. Semicolons remain invalid per the lexical grammar. The formatter's wrapping and parentheses policy is TOOL-FMT-01. Self-invoking anonymous functions are a possible future alternative to standalone blocks under PAR-FN-01, not approved here.

### PAR-LAYOUT-01 — Recognize recoverable declarative layout

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Code blocks and data structures](../spec/syntax-and-types.md#code-blocks-and-data-structures)
- **Done when:** Nominal constructor lists and `when`/`where` branch lists are parsed using syntactic structure, type/pattern constraints when applicable, and layout without requiring a fixed number of leading spaces. Inconsistent indentation is normalized by the formatter when ownership is unique. If multiple plausible interpretations remain and cannot be safely resolved, compilation reports the ambiguity and explains the invalid interpretations rather than selecting one or automatically modifying source.
- **Notes:** Approved 2026-10-21. Full grammar and proof of the indentation-safety property remain implementation work; does not claim every possible layout issue is already characterized (LEX-IND-02).

### PAR-EXPR-01 — Expression grammar and precedence

- **Status:** Implemented
- **Spec:** [syntax-and-types.md — Value expression grammar and precedence](../spec/syntax-and-types.md#value-expression-grammar-and-precedence)
- **Done when:** Ordinary value expressions parse with this precedence from highest to lowest: postfix member access/call (`.`, `()`), prefix unary (`!`, `-`), multiplicative (`*`, `/`, `%`), additive (`+`, binary `-`), comparisons (`<`, `<=`, `>`, `>=`, `==`, `!=`), `&&`, `||`, then right-associative `=>`. Postfix operations chain left-to-right; `*`, `/`, `%`, `+`, binary `-`, `&&`, and `||` are left-associative; comparisons are non-associative. Calls use parentheses only: `f(x)` and `f()` are valid, while whitespace application `f x` is invalid.
- **Evidence:** `tests/parser/main.rs::postfix::calls_use_parentheses`, `tests/parser/main.rs::postfix::whitespace_application_is_rejected`, `tests/parser/main.rs::postfix::member_access_and_calls_chain_left_to_right`, `tests/parser/main.rs::postfix::keyword_spellings_are_member_names`, `tests/parser/main.rs::prefix::prefix_operators_bind_looser_than_postfix`, `tests/parser/main.rs::prefix::prefix_operators_bind_tighter_than_binary`, `tests/parser/main.rs::binary::multiplicative_binds_tighter_than_additive`, `tests/parser/main.rs::binary::arithmetic_is_left_associative`, `tests/parser/main.rs::binary::comparisons_bind_looser_than_arithmetic`, `tests/parser/main.rs::binary::comparisons_are_non_associative`, `tests/parser/main.rs::binary::and_binds_tighter_than_or`, `tests/parser/main.rs::binary::logical_operators_are_left_associative`, `tests/parser/main.rs::lambdas::approved_parameter_forms`, `tests/parser/main.rs::lambdas::arrow_is_right_associative_and_lowest`, `tests/parser/main.rs::lambdas::lambdas_as_operands_and_arguments`, `tests/parser/main.rs::lambdas::non_parameter_left_operands_are_rejected`, `tests/parser/main.rs::lambdas::nested_groups_and_lambda_heads`, `tests/parser/main.rs::line_breaks::block_comment_line_breaks_are_not_joined`, `tests/parser/main.rs::primaries::type_operators_are_not_value_operators`
- **Notes:** `&&` and `||` short-circuit. Type-expression `|` and `&` are not value operators and remain PAR-TYPE-01; `/` is division in value and type computations; set difference is `\` (PAR-TYPE-08). Euclidean modulo is separately approved but its operator spelling is still open (PAR-EXPR-03); when introduced, it uses the multiplicative tier and left associativity. Pipe syntax and precedence remain PAR-PIPE-01. Validation `as?` applies after ordinary arithmetic unless its expression is parenthesized (PAR-VAL-01); its other grammar remains dedicated. The implementation (`natcode::parser::parse_expression`) parses comma-separated positional call arguments and the approved lambda parameter forms with expression bodies. A complete operand left of `=>`, as in `a || x => y`, is rejected because it is not a parameter list. Member names may use keyword spellings (`record.type`), as `lexical.md` permits for property names; whether `true` and `false` may too is PAR-EXPR-04. A line break inside a block comment ends a line like an explicit one (LEX-COM-04). Line continuation (PAR-BLK-02), lambda block bodies (PAR-BLK-01), labeled arguments and immediately invoked lambdas (PAR-FN-01), `as?` (PAR-VAL-01, PAR-VAL-02), `_` placeholders (PAR-EXPR-02), object literals, and `when` are not parsed yet.

### PAR-EXPR-02 — Expression placeholders and partial application

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Runtime validation](../spec/syntax-and-types.md#runtime-validation)
- **Questions:**
  1. May standalone `_` appear in ordinary value expressions to introduce partial application, including forms such as `_ as? Positive`?
  2. If so, what expression owns a placeholder, and how do multiple or nested placeholders introduce parameters?
  3. How does placeholder application interact with future pipe syntax?
- **Notes:** Standalone `_` already has a lexical token and wildcard meanings in patterns. Those existing meanings do not decide its ordinary-expression semantics.

### PAR-EXPR-03 — Euclidean modulo operator spelling

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Numeric arithmetic operators](../spec/syntax-and-types.md#numeric-arithmetic-operators)
- **Questions:**
  1. What spelling should the approved Euclidean modulo operator use, distinct from truncated-remainder `%` and existing syntax?
- **Notes:** The operation's numeric behavior is approved in TYP-ARITH-03, including fractional operands and a nonnegative result for nonzero positive or negative divisors. Its precedence and left associativity match `*`, `/`, and `%` (PAR-EXPR-01). `MOD` in explanatory examples is only a placeholder, not a token or keyword.

### PAR-EXPR-04 — Boolean literal spellings as member names

- **Status:** Proposed
- **Spec:** [lexical.md — Tokens](../spec/lexical.md#tokens)
- **Done when:** The language owner has decided whether `true` and `false` may follow `.` as property names (for example `settings.true`), and the parser accepts or rejects them accordingly.
- **Notes:** Raised while implementing PAR-EXPR-01. `lexical.md` permits keyword spellings as object property names, and the parser accepts the nine reserved keywords after `.` (`record.type`). It lists `true` and `false` separately as Boolean literals, not keywords, so it does not say whether the same rule covers them. The parser currently rejects `record.true`. Possible answers, none selected: (a) accept both literal spellings after `.`, consistent with every reserved spelling being usable as a property name; (b) keep rejecting them, so these spellings are always literals and such properties are reachable only by other means, such as computed keys (PAR-OBJ-01). The same decision likely applies to static keys in object literals (PAR-OBJ-01) and record types (PAR-TYPE-09).

### PAR-FN-01 — Function and lambda syntax

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Function parameters and labeled calls](../spec/syntax-and-types.md#function-parameters-and-labeled-calls)
- **Questions:**
  1. What is the remaining full grammar for optional, rest, destructured, and generic parameters, and for return annotations where applicable?
  2. Should explicitly unlabeled `fn` parameters (Swift-style `_ value: Type`) be supported? Decision deferred.
  3. What remaining function-value/call grammar needs to be established?
- **Notes:** Approved portions: named functions use `fn`; parameters may omit annotations (PAR-FN-04) or use `name: Type` and defaults `name: Type = expression`, with defaults allowed before required parameters. Calls to named functions use `label: expression`, with arbitrary argument order. Separate external labels and internal bindings use `external as internal: Type`, optionally with a default. Lambdas use positional unannotated parameters (`x => x * x`, `(x, y) => x + y`, `() => 42`), with parentheses optional only for a sole untyped parameter. Standalone lambda declarations such as `double = x => x * 2` are invalid, but callbacks and returning lambdas are valid; `fn` functions can be passed as positional callbacks.


### PAR-FN-04 — Optional annotations for implicitly polymorphic parameters

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Function parameters and labeled calls](../spec/syntax-and-types.md#function-parameters-and-labeled-calls)
- **Done when:** A named `fn` parameter permits an omitted type annotation; its effective constraint is then `Any`. Present annotations constrain arguments but do not widen their inferred argument types.
- **Notes:** Inference of implicit independent type variables is TYP-FN-06. The remaining optional/rest/destructured function parameter grammar stays PAR-FN-01.

### PAR-FN-02 — Dedicated function declaration keyword

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Declaration keywords and expression results](../spec/syntax-and-types.md#declaration-keywords-and-expression-results)
- **Done when:** Named function declarations are introduced by `fn`; ordinary value bindings remain keyword-free.
- **Notes:** Parameter and body syntax remain PAR-FN-01.

### PAR-FN-03 — Implicit function results and no early return

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Declaration keywords and expression results](../spec/syntax-and-types.md#declaration-keywords-and-expression-results)
- **Done when:** A function's result is the value of its final expression; an explicit `return` or early-return statement is not part of Natcode grammar.
- **Notes:** Exact function body/block grammar remains PAR-BLK-02 and PAR-FN-01.

### PAR-COND-01 — Remaining when branch grammar

- **Status:** Open question
- **Spec:** [syntax-and-types.md — When expressions](../spec/syntax-and-types.md#when-expressions)
- **Questions:**
  1. What additional details govern branch list boundaries, nested expression parsing, and syntax-error recovery?
  2. Are pattern guards supported and how are they spelled?
- **Notes:** Approved body forms are `pattern => expression` and `pattern => { … }`; blocks contain multiple expressions/declarations and yield the final expression. A nested `when` remains one expression without requiring braces (PAR-BLK-01). `=> ({ key: value })` disambiguates an anonymous object (PAR-OBJ-02).

### PAR-COND-02 — Unified `when` form

- **Status:** Approved
- **Spec:** [syntax-and-types.md — When expressions](../spec/syntax-and-types.md#when-expressions)
- **Done when:** Both `when subject` and subjectless `when` produce values; the former matches its subject and the latter tests Boolean conditions in order. No separate `if`/`else` syntax.

### PAR-COND-03 — Mixed patterns and predicates

- **Status:** Approved
- **Spec:** [syntax-and-types.md — When expressions](../spec/syntax-and-types.md#when-expressions)
- **Done when:** Subject-based `when` accepts mixed literal, constructor, and unary Boolean predicate function branches; a predicate receives the subject implicitly. Subjectless branches evaluate written Boolean expressions.

### PAR-COND-04 — Fat-arrow branch separator

- **Status:** Approved
- **Spec:** [syntax-and-types.md — When expressions](../spec/syntax-and-types.md#when-expressions)
- **Done when:** `=>` separates a `when` branch pattern or condition from its result and is shared with predicate-bearing `where` branches.

### PAR-OBJ-01 — Object literal grammar

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Code blocks and data structures](../spec/syntax-and-types.md#code-blocks-and-data-structures)
- **Done when:** Object literals support `{}`; comma-separated properties with an optional trailing comma; explicit `key: expression` properties; identifier shorthand such as `{ name }`; static identifier, string-literal, and numeric-literal keys; and computed `[expression]: expression` keys. Commas are required between entries even across line breaks. Shorthand is identifier-only. Repeating the same written static key in one literal is a compile error.
- **Notes:** Ordinary object properties use `:` rather than `=` (PAR-OBJ-02). Dynamic-key type representation and collision-sensitive inference are TYP-OBJ-01. Object spread is PAR-OBJ-03. The distinct export-alias syntax is `export { localName as publicName }` (PAR-EXP-02).

### PAR-OBJ-02 — Object property colons and arrow-body disambiguation

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Code blocks and data structures](../spec/syntax-and-types.md#code-blocks-and-data-structures)
- **Done when:** Object literal properties use `name: expression`, not `name = expression`. An unparenthesized `{}` immediately following `=>` is an executable block; to return an anonymous object directly, use `=> ({ name: expression })`. Object literals in ordinary expression positions, such as `user = { name: "Ada" }`, do not require parentheses.
- **Notes:** Approved 2026-10-21. Does not change the separate named-import/export alias grammar (PAR-IMP-07, PAR-EXP-02).

### PAR-OBJ-03 — Object spread syntax

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Code blocks and data structures](../spec/syntax-and-types.md#code-blocks-and-data-structures)
- **Questions:**
  1. Does Natcode support object spread syntax such as `{ ...user, name: "Grace" }`?
  2. If supported, where may spread entries appear, may multiple spreads occur, and do they use the same comma-separation rules as ordinary object entries?
- **Notes:** Object-spread typing and overwrite semantics should be tracked separately from parser syntax if spread is approved.

### PAR-TYPE-01 — Remaining type-expression precedence

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Type-expression computations and record syntax](../spec/syntax-and-types.md#type-expression-computations-and-record-syntax)
- **Questions:**
  1. How do comparisons and Boolean `&&`/`||` group relative to `where` and the set operators in a single type expression?
- **Notes:** Approved type-expression forms are tracked by PAR-TYPE-08 through PAR-TYPE-12 and PAR-REF-01. Explicit generic syntax remains undecided (TYP-GEN-01, PAR-TYPE-02).

### PAR-TYPE-09 — Record and primitive literal type grammar

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Type-expression computations and record syntax](../spec/syntax-and-types.md#type-expression-computations-and-record-syntax)
- **Done when:** Numeric (including unary negative and decimal), double-quoted string, and Boolean literal types and parenthesized type expressions parse, along with record types using `{}`, `key: Type` entries, comma separators, optional trailing comma, and identifier/string/numeric static keys.
- **Notes:** Computed keys and their type representation remain TYP-OBJ-01.

### PAR-TYPE-10 — Numeric and Boolean computations in type grammar

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Type-expression computations and record syntax](../spec/syntax-and-types.md#type-expression-computations-and-record-syntax)
- **Done when:** Type computations parse numeric arithmetic, comparisons, and `&&`, `||`, `!` alongside set `|`, `&`, `\`, `~`. Boolean operand validity is TYP-BOOL-01. Euclidean modulo's eventual syntax depends on PAR-EXPR-03.
- **Notes:** Relative grouping of comparisons/Boolean and set operations remains PAR-TYPE-01.

### PAR-TYPE-11 — Approved type precedence tiers

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Type-expression computations and record syntax](../spec/syntax-and-types.md#type-expression-computations-and-record-syntax)
- **Done when:** Type expressions bind grouping/application, prefix `~`/`!`/unary `-`, multiplicative, additive, postfix `where`, `&` and `\` together (left associative), then `|` in that order, without assigning the unresolved comparison/Boolean tiers.
- **Notes:** The approved `~Number where ...` means `(~Number) where ...`, not `~(Number where ...)`.

### PAR-TYPE-12 — Value binding references in type computations

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Type-expression computations and record syntax](../spec/syntax-and-types.md#type-expression-computations-and-record-syntax)
- **Done when:** A value-binding name may be used as an operand in a type computation; its inferred type and dependencies are resolved by type analysis, without enabling arbitrary effectful execution.
- **Notes:** The preservation of relationships between repeated and intermediate bindings is TYP-COMP-02.

### PAR-TYPE-02 — Remaining type declaration grammar

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Questions:**
  1. What additional grammar is needed for generic parameters, named fields, recursive declarations, and advanced variants beyond the approved positional and algebraic constructor-list forms?

### PAR-TYPE-03 — Transparent and nominal type declarations

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Declaration keywords and expression results](../spec/syntax-and-types.md#declaration-keywords-and-expression-results)
- **Done when:** `type Name = expression` declares a transparent name; `unique type Name(...)` or `unique type Name` followed by constructor branches introduces a nominal identity. Bare `type Name(...)` and `unique type Name = expression` are invalid. Remaining grammar is PAR-TYPE-02.

### PAR-TYPE-04 — Positional nominal constructors

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** `unique type UserId(Number)` and `unique type Point(Number, Number)` parse as nominal type declarations with positional constructor payloads.

### PAR-TYPE-05 — Alias declarations

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** `type Identifier = UserId | ProductId` and `type Some = Option.Some` parse as transparent declarations, distinguishable from nominal `unique type` declarations. Constructor-symbol names remain callable and valid in constructor patterns.

### PAR-TYPE-06 — Indented constructors and qualified lookup

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** `unique type Option` with `Some(Any)` and `None` on subsequent constructor-list lines parses as a nominal algebraic declaration regardless of inconsistent leading-space alignment, and `Option.Some` resolves as a qualified constructor name. Structural ambiguity is diagnosed; formatting normalizes the constructor list.

### PAR-TYPE-07 — Constructor restrictions in type expressions

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** `Value("one" | "two")` parses in a type context as a restriction on an existing nominal constructor, while `Value("one")` in a value context constructs a value.

### PAR-TYPE-08 — Set-difference type operator

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Union and intersection types](../spec/syntax-and-types.md#union-and-intersection-types)
- **Done when:** `A \ B` parses as set difference in a type expression, at the same left-associative precedence as `&`, higher than `|` and lower than `where`. In type computations `A / B` denotes arithmetic division, not difference.
- **Notes:** Replaces the earlier `/` difference spelling, approved 2026-10-25.

### PAR-UPD-01 — Dotted update syntax

- **Status:** Open question
- **Spec:** [modules.md — First-class namespaces and persistent updates](../spec/modules.md#first-class-namespaces-and-persistent-updates)
- **Questions:**
  1. What is the grammar of dotted and nested property updates such as `Math.PI = value`?

### PAR-EXT-01 — Extension and constructor association syntax

- **Status:** Open question
- **Spec:** [modules.md — What is a module?](../spec/modules.md#what-is-a-module)
- **Questions:**
  1. What syntax associates independently defined functions with a type?

### PAR-PIPE-01 — Piping syntax

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Inference-first philosophy](../spec/syntax-and-types.md#inference-first-philosophy)
- **Questions:**
  1. Piping is a design goal. Is there a pipe operator, and what is its syntax and precedence?
  2. How does piping interact with expression placeholders and designated error values, including any short-circuiting behavior?

### PAR-VAL-01 — Runtime-validation expression

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Runtime validation](../spec/syntax-and-types.md#runtime-validation)
- **Done when:** `value as? TypeExpression` parses as a runtime-validation expression whose right operand is one complete type expression; for example, `x as? Number where n => n > 0` takes the whole refinement as its target. Arithmetic binds before validation: `a / b as? T` validates the division expression, while `a / (b as? T)` and `(a as? T) / b` require parentheses to use validation as an arithmetic operand. `as?` is the single token defined by LEX-OP-05.
- **Notes:** The remaining precedence of ordinary value operators stays PAR-EXPR-01. General `_` partial-application grammar is PAR-EXPR-02.

### PAR-VAL-02 — Precedence of `as?` relative to comparison, Boolean, and lambda operators

- **Status:** Proposed
- **Spec:** [syntax-and-types.md — Runtime validation](../spec/syntax-and-types.md#runtime-validation)
- **Done when:** The language owner has decided how `as?` groups relative to comparisons, `&&`, `||`, and `=>`, and where its right-hand type expression ends, and the parser groups accordingly.
- **Notes:** Raised while implementing PAR-EXPR-01. PAR-VAL-01 only settles that arithmetic binds before `as?` (`a / b as? T` means `(a / b) as? T`). Unresolved examples: `a < b as? T` could mean `(a < b) as? T` or `a < (b as? T)`; `ok && x as? T` could mean `(ok && x) as? T` or `ok && (x as? T)`; and in `x as? T && ok`, it is unclear whether `&& ok` belongs to the type expression or applies to the validation result, because the right operand is a complete type expression whose Boolean grammar is itself open (PAR-TYPE-01). Possible answers, none selected: (a) `as?` binds between additive operators and comparisons; (b) `as?` binds below comparisons but above `&&`; (c) mixing `as?` with comparison or Boolean operators without parentheses is a compile error. The parser does not parse `as?` yet.

### PAR-PAT-01 — Pattern matching syntax

- **Status:** Open question
- **Spec:** [syntax-and-types.md — When expressions](../spec/syntax-and-types.md#when-expressions)
- **Questions:**
  1. What is the detailed grammar for constructor-destructuring patterns and any additional literal, destructuring, and other pattern forms within subjectful `when`? Constructor destructuring is approved.
  2. Where may `_` appear as a wildcard within compound patterns?

### PAR-PAT-02 — Shared pattern language for `when` and `where`

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Where patterns and filtering](../spec/syntax-and-types.md#where-patterns-and-filtering)
- **Done when:** Every pattern form defined by PAR-PAT-01 has the same pattern meaning in subjectful `when` and `where`, subject to each construct's different branch semantics.

## Refinement type grammar

### PAR-REF-01 — Postfix `where` type operator and precedence

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Where placement and precedence](../spec/syntax-and-types.md#where-placement-and-precedence)
- **Done when:** `where` is accepted wherever a type expression is accepted; arithmetic and prefix `~`, `!`, and unary `-` bind more tightly than `where`; `where` binds more tightly than set `|`, `&`, and `\`. Parentheses select larger left operands. Direct chains such as `T where ... where ...` are rejected, and each `where` begins on the physical line of its preceding type/constructor (or closing parenthesis). Comparison/Boolean precedence remains PAR-TYPE-01.
- **Notes:** Extended with approved complement and arithmetic precedence 2026-10-25.

### PAR-REF-02 — Refinement binding and destructuring forms

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Binding and destructuring the input](../spec/syntax-and-types.md#binding-and-destructuring-the-input)
- **Done when:** A single identifier binds the whole candidate value or the single exposed parameter; positional `(x, y, ...)` destructuring is accepted only when the left type has one unambiguous constructor shape; mixed nominal constructors require explicit constructor patterns.

### PAR-REF-03 — Applied-constructor restriction boundary

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Where placement and precedence](../spec/syntax-and-types.md#where-placement-and-precedence)
- **Done when:** A direct form such as `Position(0, Number) where ...` or `Position(Number, Number) where ...` is rejected, while `Position(0, Number where ...)` and a named specialized alias followed by `where` are accepted.

### PAR-REF-04 — Refinement branch surface forms

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Where patterns and filtering](../spec/syntax-and-types.md#where-patterns-and-filtering)
- **Done when:** Predicate branches use `pattern => BooleanExpression`; bare constructor branches such as `Success` are accepted as inclusion sugar; explicit `true`/`false` and final `_` branches parse; `where` itself does not require an exhaustive final branch.

### PAR-REF-05 — Refinements in nominal declarations

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Restrictions in nominal declarations](../spec/syntax-and-types.md#restrictions-in-nominal-declarations)
- **Done when:** `where` can appear inside an individual constructor parameter type or after a positional or algebraic constructor declaration to restrict that constructor; a `where` on a nominal declaration with multiple constructors as a whole is rejected. Alias-level `where` filtering of an existing algebraic type remains valid.

## Errors

### PAR-ERR-01 — Recoverable syntax errors with locations

- **Status:** Proposed
- **Spec:** None
- **Done when:** Invalid syntax reports diagnostics with spans, parsing recovers to report further errors, and the parser never crashes on any token stream.
