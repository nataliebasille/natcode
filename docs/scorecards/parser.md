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
- **Done when:** `x: number = 42` parses with the constraint attached to the declaration.

### PAR-DECL-03 — Initializer required

- **Status:** Approved
- **Spec:** [variables.md — Declaration syntax](../spec/variables.md#declaration-syntax)
- **Done when:** `x: number` without `= expression` is rejected.

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
- **Done when:** `{ localName = publicName }` parses with the local name on the left.

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
- **Done when:** `export { squareNumber = square }` parses with the public name on the left.

### PAR-EXP-03 — Export of an arbitrary expression

- **Status:** Approved
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Done when:** `export run`, `export api`, and `export makeApi()` parse as an export of an expression.

## Expressions, blocks, and other grammar

### PAR-BLK-01 — Braces are not statement blocks

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Code blocks and data structures](../spec/syntax-and-types.md#code-blocks-and-data-structures)
- **Done when:** Nested scopes are delimited by indentation, and `{ … }` is only ever parsed as a data construct.

### PAR-BLK-02 — Statement and block syntax

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Code blocks and data structures](../spec/syntax-and-types.md#code-blocks-and-data-structures)
- **Questions:**
  1. Which constructs introduce an indented block?
  2. What may a block contain, and what is its value?

### PAR-EXPR-01 — Expression grammar and precedence

- **Status:** Open question
- **Spec:** [variables.md — Declaration grammar (schematic)](../spec/variables.md#declaration-grammar-schematic)
- **Questions:**
  1. What are the precedence and associativity of `+`, `*`, `|`, `&`, `.`, calls, and `=>`?
  2. What is the function call syntax (`f(x)`, `f x`, both)?
- **Notes:** Approved examples rely on `x + 5`, `PI * 5 * 5`, `f()`, and `Math.PI` without a fixed grammar.

### PAR-FN-01 — Function and lambda syntax

- **Status:** Open question
- **Spec:** [evaluation.md — Function return inference: context-independent](../spec/evaluation.md#function-return-inference-context-independent)
- **Questions:**
  1. What is the complete grammar for functions introduced by `fn` and for lambda expressions? `x => x * x` and `() => x` are illustrative.
  2. How are parameters and their type constraints written?

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
  1. What is the exact grammar for indented branch bodies beyond the approved `=>` separator?
  2. Are pattern guards supported and how are they spelled?
- **Notes:** The branch separator itself is approved as `=>` in PAR-COND-04.

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

- **Status:** Open question
- **Spec:** [modules.md — Exported kind: object library or function executable](../spec/modules.md#exported-kind-object-library-or-function-executable)
- **Questions:**
  1. What is the object literal grammar? `{ PI = 3.14159, square = x => x * x }` is illustrative.
  2. Are computed or dynamic keys written in literals?

### PAR-TYPE-01 — Type expression grammar

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Union and intersection types](../spec/syntax-and-types.md#union-and-intersection-types)
- **Questions:**
  1. What is the grammar for record types, unions with `|`, intersections with `&`, and literal types? `{ kind: "left", value: number }` is illustrative.
  2. What is the generic parameter grammar?

### PAR-TYPE-02 — Remaining type declaration grammar

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Questions:**
  1. What additional grammar is needed for generic parameters, named fields, recursive declarations, and advanced variants beyond the approved positional and indented constructor forms?

### PAR-TYPE-03 — Dedicated named type keyword

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Declaration keywords and expression results](../spec/syntax-and-types.md#declaration-keywords-and-expression-results)
- **Done when:** Named type declarations use the `type` introducer; their remaining grammar is determined by PAR-TYPE-02.

### PAR-TYPE-04 — Positional nominal constructors

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** `type UserId(number)` and `type Point(number, number)` parse as nominal type declarations with positional constructor payloads.

### PAR-TYPE-05 — Alias declarations

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** `alias Identifier = UserId | ProductId` and `alias Some = Option.Some` parse as transparent aliases, distinguishable from nominal `type` declarations.

### PAR-TYPE-06 — Indented constructors and qualified lookup

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** `type Option` with indented `Some(any)` and `None` parses as a nominal algebraic declaration and `Option.Some` resolves as a qualified constructor name.

### PAR-TYPE-07 — Constructor restrictions in type expressions

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Nominal types, constructors, and aliases](../spec/syntax-and-types.md#nominal-types-constructors-and-aliases)
- **Done when:** `Value("one" | "two")` parses in a type context as a restriction on an existing nominal constructor, while `Value("one")` in a value context constructs a value.

### PAR-TYPE-08 — Set-difference type operator

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Union and intersection types](../spec/syntax-and-types.md#union-and-intersection-types)
- **Done when:** `A / B` parses in type-expression context as binary set difference; its precedence relative to `where` follows PAR-REF-01, while the remaining set-operator precedence table stays PAR-TYPE-01.

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
- **Done when:** `where` is accepted wherever a type expression is accepted, binds tighter than `|`, `&`, and `/`, parentheses can select a larger left operand, and a direct chain such as `T where ... where ...` is rejected.

### PAR-REF-02 — Refinement binding and destructuring forms

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Binding and destructuring the input](../spec/syntax-and-types.md#binding-and-destructuring-the-input)
- **Done when:** A single identifier binds the whole candidate value or the single exposed parameter; positional `(x, y, ...)` destructuring is accepted only when the left type has one unambiguous constructor shape; mixed nominal constructors require explicit constructor patterns.

### PAR-REF-03 — Applied-constructor restriction boundary

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Where placement and precedence](../spec/syntax-and-types.md#where-placement-and-precedence)
- **Done when:** A direct form such as `Position(0, number) where ...` or `Position(number, number) where ...` is rejected, while `Position(0, number where ...)` and a named specialized alias followed by `where` are accepted.

### PAR-REF-04 — Refinement branch surface forms

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Where patterns and filtering](../spec/syntax-and-types.md#where-patterns-and-filtering)
- **Done when:** Predicate branches use `pattern => BooleanExpression`; bare constructor branches such as `Success` are accepted as inclusion sugar; explicit `true`/`false` and final `_` branches parse; `where` itself does not require an exhaustive final branch.

### PAR-REF-05 — Refinements in nominal declarations

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Restrictions in nominal declarations](../spec/syntax-and-types.md#restrictions-in-nominal-declarations)
- **Done when:** `where` can appear inside an individual constructor parameter type, after a positional or algebraic constructor declaration, and after an algebraic constructor list to restrict the nominal type as a whole.

## Errors

### PAR-ERR-01 — Recoverable syntax errors with locations

- **Status:** Proposed
- **Spec:** None
- **Done when:** Invalid syntax reports diagnostics with spans, parsing recovers to report further errors, and the parser never crashes on any token stream.
