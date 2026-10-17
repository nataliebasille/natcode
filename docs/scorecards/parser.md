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

### PAR-COND-01 — Match branch grammar

- **Status:** Open question
- **Spec:** [syntax-and-types.md — Match expressions](../spec/syntax-and-types.md#match-expressions)
- **Questions:**
  1. What is the exact grammar for branch patterns, condition tests, separators, and indented bodies? `=>` remains illustrative.
  2. What are the exhaustive-match requirements and behavior when no branch matches?
  3. Are pattern guards supported, and how are they spelled?

### PAR-COND-02 — Unified matching and conditional form

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Match expressions](../spec/syntax-and-types.md#match-expressions)
- **Done when:** Natcode accepts `match expression` for matching patterns against a subject and subjectless `match` for testing Boolean conditions in order; both are value-producing expressions, and `if`/`else` conditionals are not supported.
- **Notes:** The complete branch grammar and exhaustiveness behavior remain PAR-COND-01.

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

### PAR-TYPE-02 — Type declarations

- **Status:** Open question
- **Spec:** [variables.md — Identifier categories](../spec/variables.md#identifier-categories)
- **Questions:**
  1. After the approved `type` introducer, how are named nominal types, generic parameters, and constructors declared? `type Point(x: number, y: number)` is illustrative.
  2. What syntax constructs a value of a nominal type from an underlying value, if such a form is allowed?

### PAR-TYPE-03 — Dedicated named type keyword

- **Status:** Approved
- **Spec:** [syntax-and-types.md — Declaration keywords and expression results](../spec/syntax-and-types.md#declaration-keywords-and-expression-results)
- **Done when:** Named type declarations use the `type` introducer; their remaining grammar is determined by PAR-TYPE-02.

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
- **Spec:** [syntax-and-types.md — Match expressions](../spec/syntax-and-types.md#match-expressions)
- **Questions:**
  1. Which constructor, literal, destructuring, and other pattern forms are supported inside the approved subjectful `match` expression?
  2. Where may `_` appear as a wildcard within compound patterns?

## Errors

### PAR-ERR-01 — Recoverable syntax errors with locations

- **Status:** Proposed
- **Spec:** None
- **Done when:** Invalid syntax reports diagnostics with spans, parsing recovers to report further errors, and the parser never crashes on any token stream.
