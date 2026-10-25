# Broader syntax and type-system decisions

Item status and open questions for this file are tracked in the [parser](../scorecards/parser.md), [types](../scorecards/types.md), and [tooling](../scorecards/tooling.md) scorecards.

## Code blocks and data structures

- **Arbitrary executable scopes use braces `{}`**, including named function bodies and multi-expression arrow and `when` branch bodies (PAR-BLK-01). Indentation alone does not create an executable scope.
- A single-expression arrow or `when` branch body can omit braces. Braced blocks yield their final expression; no explicit `return` is required or supported (PAR-FN-03).
- **Algebraic constructor lists and `when`/`where` branch lists** retain newline-based, layout-sensitive presentation. Exact indentation width is not a syntax requirement: the parser recovers unambiguous ownership from grammar and applicable constraints, and rejects unresolved ambiguity rather than guessing (LEX-IND-01, PAR-LAYOUT-01). These lists can occur inside braces and parentheses.
- **Executable statement separation (PAR-BLK-02):** Within an executable body, local bindings and expressions are separate statements. Each statement begins on a new physical line; two complete statements cannot share a physical line. Semicolons are not supported. The final expression supplies the block value, without `return` (PAR-FN-03).
- **Expression continuation (PAR-BLK-02):** A statement may span physical lines when a trailing binary operator still needs its right operand (for example, `first +\n    second`) or when an expression is explicitly grouped in parentheses. A complete expression ends at the newline; an operator at the start of the following line cannot retroactively continue it. No explicit backslash continuation exists. Continuation cannot change ownership of a `when`/`where` branch or a nominal constructor; unresolved ambiguity is a compile error (LEX-IND-02).
- **Block placement (PAR-BLK-02):** Executable braces are permitted only as approved named-function bodies, lambda bodies, and `when` branch bodies. Standalone `{ ... }` executable expressions are not supported. In ordinary expression positions, braces follow object literal grammar (PAR-OBJ-01/02). A possible immediately invoked lambda form is future PAR-FN-01 work, not an approved substitute.
- Blank and comment-only lines never change ownership.

The following body form is approved; complete named-function parameter grammar remains PAR-FN-01:

```natcode
fn calculate(x) {
    doubled = x * 2
    doubled + 10
}
```

An arrow or `when` branch may return a single expression directly, including a nested `when`; a multi-expression body uses `{}`:

```natcode
result = when input
    Option.Some(value) => value + 1
    Option.None => {
        fallback = 0
        fallback
    }
```

Objects also use braces, but **object literal properties use `:`**, while ordinary value declarations use `=`. Immediately after `=>`, unparenthesized `{}` means an executable block, so direct anonymous-object results must be parenthesized (PAR-OBJ-02):

```natcode
user = { name: "Ada" }
getUser = () => ({ name: "Ada" })
getName = () => {
    name = "Ada"
    name
}
```

Object literals may be empty. Non-empty entries are comma-separated even when written on separate lines, and a trailing comma is optional. Static keys may be identifiers, string literals, or numeric literals. Identifier shorthand is supported, so `{ name }` means `{ name: name }`. Computed keys use `[expression]: value`:

```natcode
key = getKey()

user = {
    name: "Ada",
    "display-name": "Ada",
    123: "numeric key",
    [key]: 36,
}
```

Repeating the same written static key in one object literal is a compile error. The type representation of dynamic computed keys, including possible collisions with statically known properties and the relationship between objects and `Map`, remains TYP-OBJ-01. Object spread syntax is tracked separately by PAR-OBJ-03.

Exported-object forms such as `export { value }` and export aliases with `=` are separate grammar, unaffected by the object-literal property rule.

## Canonical formatting

Natcode has an **opinionated built-in formatter** (`natcode fmt`) and a non-mutating check mode (`natcode fmt --check`) without a separate lint or Prettier dependency (TOOL-FMT-01). The compiler accepts flexible indentation widths for recoverable structures, and the formatter produces **four spaces per level** as the canonical presentation. Compilation does not reformat files.

The formatter uses parsed structure and never changes block or branch ownership. It preserves explicitly written executable braces, including braces around a single expression, and expands single-line braced executable bodies onto separate lines rather than removing their braces:

```natcode
// Written
Result.Success(value) => { value + 1 }

// Canonical
Result.Success(value) => {
    value + 1
}
```

For continued expressions (PAR-BLK-02), formatting is based on the parsed expression: short expressions collapse onto one line, and long expressions wrap after binary operators, with continued operands indented four spaces beyond their containing statement. Explicit grouping parentheses are preserved even if their contents fit on one line. The exact wrap-width threshold remains a formatter implementation choice. Formatting must not join or split statements, reassign branches, or change expression meaning.

When syntax unambiguously determines constructor or branch membership, the formatter corrects alignment. If multiple interpretations remain and the compiler cannot safely determine ownership, it reports a compilation error with actionable diagnostics; it does not apply a speculative fix. Full proof of indentation safety and remaining error-recovery details are tracked separately (PAR-LAYOUT-01, LEX-IND-02).

## Value expression grammar and precedence

Natcode uses parenthesized function calls only: `f(x)`, `f()`, and chained forms such as `foo.bar(x).baz()` are valid. Whitespace application such as `f x` is invalid.

Ordinary value-expression operators bind from highest to lowest as follows:

| Precedence | Form | Associativity |
| --- | --- | --- |
| 1 | member access `.`, call `()` | postfix, chained left-to-right |
| 2 | unary `!`, unary `-` | prefix |
| 3 | `*`, `/`, `%` | left |
| 4 | `+`, binary `-` | left |
| 5 | `<`, `<=`, `>`, `>=`, `==`, `!=` | non-associative |
| 6 | `&&` | left, short-circuiting |
| 7 | `||` | left, short-circuiting |
| 8 | `=>` | right |

Thus `a + b * c` means `a + (b * c)`, `a - b - c` means `(a - b) - c`, `a * b / c` means `(a * b) / c`, `!ready && enabled` means `(!ready) && enabled`, and `x => y => x + y` means `x => (y => x + y)`. Comparison operators do not chain: `a < b < c` is invalid; write `a < b && b < c`.

Member access and calls form one postfix layer, so `foo.bar(x).baz()` groups as `(((foo.bar)(x)).baz)()`. Type-expression `|` and `&` are separate union/intersection operators and are not ordinary value operators. Pipe syntax and precedence remain open under PAR-PIPE-01. `/` denotes numeric division in both value expressions and type-level computations; type set difference uses `\` (PAR-TYPE-08). Validation with `as?` applies to the complete preceding arithmetic expression unless parenthesized as an operand (PAR-VAL-01); its right-hand type expression retains its own grammar.

## Strict Boolean operations

Boolean logic has no implicit truthiness: `&&`, `||`, and prefix `!` require operands assignable to `Boolean`, and each produces only Boolean values. `&&` and `||` short-circuit; they do not return their original operands as JavaScript operators can. A union with any non-Boolean possibility is an invalid Boolean operand and causes a compile error, rather than silently discarding that possibility. Known results retain literal `true` or `false` types. Subjectless `when` conditions and `where` predicates must likewise produce Boolean values (TYP-BOOL-01, RUN-BOOL-01).

Prefix `~` is reserved for type complement; Boolean NOT remains `!` (TYP-UNI-05).


## Numeric arithmetic operators

- Binary `-` subtracts numbers, with the same precedence as `+` and left associativity (PAR-EXPR-01).
- `/` performs **true division** both on runtime numbers and in type-level numeric computations; `5 / 2` evaluates to `2.5` rather than `2` (TYP-ARITH-01). Type set difference instead uses `\`.
- `%` is **truncated remainder**, with quotient rounded toward zero, including fractional operands: `-5 % 3` is `-2`, `5 % -3` is `2`, and `5.5 % 2` is `1.5` (TYP-ARITH-02).
- A **second infix operator** computes **Euclidean modulo** on fractional as well as integer operands. For a nonzero divisor `b`, its result is nonnegative and smaller than `abs(b)`, even when `b` is negative. This operator is left-associative at the same precedence as `*`, `/`, and `%` (TYP-ARITH-03).
- The Euclidean operator's **spelling is not yet chosen** (PAR-EXPR-03); the notation `MOD` in examples is illustrative and is not a lexer token or keyword.

Division, truncated remainder, and Euclidean modulo require a **provably nonzero divisor** (TYP-ARITH-04). A zero literal is rejected at compile time; an unknown numeric divisor is rejected unless the compiler has established that it cannot be zero, for example through a refinement or a successful runtime `as?` check. That validator's possible `ValidationError` is propagated through otherwise valid arithmetic under the approved error-propagation principle (TYP-ERR-03); exact propagation mechanics remain open in TYP-ERR-01. A proof failure is not silently treated as false.

The numeric representation, result type when combining `Integer` and `Number`, rounding, overflow, and non-finite-value behavior remain open in TYP-BLT-05 and TYP-BLT-06.

## When expressions

Natcode uses one `when` keyword for both structural pattern matching and conditional branching (PAR-COND-02). It does not have separate `if`/`else` or `case` control-flow constructs.

- **Subjectful** `when expression` selects a branch by matching patterns against that subject.
- **Subjectless** `when` evaluates Boolean branch conditions in source order and selects the first true condition.
- Both forms are expressions that produce the selected branch's value.

The fat arrow `=>` is the approved separator between a branch pattern or condition and its result (PAR-COND-04). Each branch may produce a single expression without braces, including another `when`; branches with multiple declarations or expressions use `{}` and yield the final expression. Explicit braces around a single expression remain valid (PAR-BLK-01). The full pattern and guard grammar remains open (PAR-COND-01, PAR-PAT-01):

```natcode
result = when option
    Some(value) => value
    None => 0

size = when
    x > 10 => "large"
    x > 5 => "medium"
    _ => "small"
```

Subject-based branches may mix literals, nominal constructor patterns, and references to user-defined unary Boolean predicate functions. A predicate function receives the subject as its argument; subjectless branches evaluate Boolean expressions as written. Branches are processed in source order.

Both forms must be exhaustive (TYP-PAT-01). The remaining possible value set must be provably `Empty` after the branches, otherwise a final `_` is required. `_` stands for the remaining universal case, may occur **at most once**, and must be **last**. A provably redundant `_` or any other provably unreachable `when` branch is a **compilation error**, not a warning. Restricted input types participate in this calculation, so constructors already eliminated from the subject type do not need branches. Arbitrary predicates do not establish exhaustiveness unless their coverage can be proven. In ambiguous layout, diagnostics should explain why plausible branch ownership interpretations fail; no layout fix is applied by guessing. Pattern guards and the complete compound-pattern grammar remain open.

## Declaration keywords and expression results

- `fn` introduces named function declarations (PAR-FN-02). Core parameter, label, and default forms are approved below; remaining grammar stays PAR-FN-01.
- Functions return the value of their final expression; there is no `return` statement or explicit early return (PAR-FN-03).
- `unique type` introduces a fresh **nominal** type identity (PAR-TYPE-03, TYP-NOM-01), even when its constructor payload types match another type.
- `type Name = expression` names an existing type expression or constructor symbol transparently, without creating a new nominal identity (PAR-TYPE-05, TYP-ALI-01, TYP-ALI-02). Constructor-symbol names retain construction and pattern behavior. The full expression grammar remains PAR-TYPE-01.
- `unique type Name(...)` and `unique type Name` with constructor branches introduce nominal constructors (PAR-TYPE-04, PAR-TYPE-06). Bare `type Name(...)` and `unique type Name = expression` are invalid. Further declaration and generic grammar remains open (PAR-TYPE-02).
- Ordinary variable bindings continue to use keyword-free `name = expression` declarations (PAR-DECL-01).

## Function parameters and labeled calls

Named functions may omit parameter type annotations. Every parameter is implicitly polymorphic and has its own independent type variable, constrained by its annotation or by `Any` when omitted (PAR-FN-04, TYP-FN-06). An explicit annotation constrains acceptable arguments but does not widen their exact types. Calls to a named `fn` declaration use `label: expression` (not `label = expression`), and labeled arguments may appear in any order. Parameter defaults use `=` and may precede required parameters (PAR-FN-01).

```natcode
fn greet(first: String = "Ada", last: String) {
    first + " " + last
}

greet(last: "Lovelace")
greet(last: "Lovelace", first: "Grace")
```

A parameter may have distinct external and internal names via `external as internal: Type`, with an optional `= default`. Callers use the external label; the body uses the internal binding. Without an alias, one parameter name serves both purposes.

```natcode
fn greet(person as name: String = "Guest") {
    "Hello, " + name
}

greet(person: "Ada")
```

Plain `as` is for renaming only, consistent with import/export aliases (PAR-IMP-07, PAR-EXP-02). It does not assert a type or convert a value. The distinct `as?` token remains the runtime-validation operator (PAR-VAL-01, LEX-OP-05).

Lambda parameters remain positional and cannot have inline type annotations. Valid parameter forms include `x => x * 2`, `(x) => x * 2`, `(x, y) => x + y`, and `() => 42`; parentheses may be omitted only around one untyped parameter. Lambdas can appear as callback arguments or function results, but cannot be bound as standalone function declarations (`double = x => x * 2` is invalid); use `fn double(x: number) { x * 2 }`. Named functions can be passed as positional callbacks. Explicitly unlabeled `fn` parameters and remaining function/lambda grammar are not settled (PAR-FN-01).

## Inference-first philosophy

Natcode seeks a TypeScript-like ergonomics with a substantially more functional model, emphasizing purity, piping, and pattern matching. These are **design goals** (see RUN-EFF-01, PAR-PIPE-01, and PAR-PAT-01).

Ordinary variable types and function return types are intended to be inferred. The optional declaration constraint `x: Type = expression` is permitted as an **assignability check** without replacing or widening the inferred type; this updates the earlier inference-only proposal.

## Nominal types, constructors, and aliases

A `unique type` declaration creates a fresh nominal identity. Constructing values of two independently declared nominal types does not make those values interchangeable, even when their payload types coincide (TYP-NOM-01).

```natcode
unique type UserId(number)
unique type ProductId(number)

userId = UserId(123)
productId = ProductId(123)
```

Nominal wrappers do **not** implicitly convert to their payload types or automatically inherit their operators (TYP-NOM-02). User-defined operator syntax and resolution remain open until function semantics are designed.

A constructor can have multiple positional parameters. The constructor form is also a pattern that binds the contents of its fields; there is no automatically generated `.value` accessor (TYP-NOM-03, TYP-NOM-04, PAR-PAT-01).

```natcode
unique type Point(number, number)

point = Point(10, 20)
coordinates = when point
    Point(x, y) => { x, y }
```

The `when` branch and record literal syntax in these examples is illustrative; the pattern-matching ability to destructure constructors is approved, while the complete grammar is PAR-PAT-01 and PAR-COND-01.

An algebraic nominal type may declare multiple constructors in a constructor list; its indentation is normalized by the formatter and does not independently establish semantic nesting. A constructor is qualified by its declaring type by default; the constructor's unqualified name is **not** automatically in scope (TYP-ADT-01, PAR-TYPE-06, RES-NAME-06).

```natcode
unique type Option
    Some(Any)
    None

present = Option.Some(42)
absent = Option.None
```

A transparent `type Name = ...` declaration can name a type expression or an existing constructor symbol. Constructor aliases preserve the original constructor identity and are usable in value construction and constructor patterns (TYP-ALI-01, TYP-ALI-02).

```natcode
type Some = Option.Some

value = Some(42)
unwrapped = when value
    Some(x) => x
```

In **type-expression context**, applying an existing nominal constructor to type arguments describes a subset of that nominal type constrained by its payload values; in **value-expression context**, applying it constructs a value (TYP-NOM-05). Omitting the arguments names the entire nominal type with no further restriction.

```natcode
unique type Value(string)
unique type Value2(number)

type Z = Value("one" | "two") | Value2(1 | 2)
type Unrestricted = Value | Value2
```

For a single-payload constructor, `Value(A | B)` represents the same set of possible values as `Value(A) | Value(B)`. For multiple arguments, correlations must be preserved: `Point(1 | 2, 3 | 4)` represents four combinations, while `Point(1, 3) | Point(2, 4)` represents only two. The complete grammar for literal type expressions is PAR-TYPE-01.

Nominal types may contain other nominal types through ordinary constructor parameters (TYP-NOM-06).

```natcode
unique type Position(number, number)
unique type Color(string)

unique type Sprint(Position, Color)
// Equivalent unrestricted parameter types:
// unique type Sprint(Position(number, number), Color(string))

sprint = Sprint(Position(10, 20), Color("red"))
```

These nested values are constructor arguments; this declaration **does not imply** implicit projection, inheritance, an interface, independent-aspect composition, or automatic reconstruction after a nested value is transformed. No separate nominal-combination operator or construct is approved. The previously explored special combination syntax is rejected in favor of ordinary nesting. How a function might preserve a containing value while transforming a nested value, versus return an unrelated value, remains an open question (TYP-FN-05).

## Refinement types with `where`

`where` is a postfix type operator that describes a **subset** of the type expression on its left (TYP-REF-01, PAR-REF-01). It does not create a nominal identity. A branch predicate is Boolean: values for which the predicate is true remain in the type, and values for which it is false are excluded.

```natcode
type Positive = number where
    x => x > 0

type AboveDiagonal = Position where
    (x, y) => x > y
```

A restricted alias remains assignable to the type it restricts. The reverse assignment is accepted only when the compiler can establish that the value satisfies the restriction (TYP-REF-11).

### Where patterns and filtering

`where` and subjectful `when` use the same pattern language (PAR-PAT-02). Literal patterns, constructor patterns, wildcards, and constructor destructuring therefore mean the same thing in both constructs. Unlike `when`, however, `where` is a filter and **does not require exhaustive branches** (TYP-REF-03). Its branch list is indentation-insensitive when branch ownership is unambiguous; ambiguous ownership is a compile error rather than a guessed autofix (PAR-LAYOUT-01).

A pattern claims the part of the input type that it matches. Its Boolean result decides which values from that matched part remain. A value whose pattern matched but whose predicate is false is excluded; it does not fall through to a later branch. Values not matched by any branch are also excluded.

```natcode
unique type Option
    Some(number)
    None

type PositiveOption = Option where
    Some(x) => x > 0
```

Here positive `Some` values remain, negative and zero `Some` values are excluded, and `None` is excluded because no branch includes it.

Ordinary branch patterns must describe disjoint regions. If two ordinary branches could match the same value, that is a compile error. `_` is special: it denotes only the region not matched by earlier branches. Consequently it must be final whenever it consumes the entire remaining region, and any branch after the remaining region is already `Empty` is a compile error.

```natcode
type KeepOtherCases = Option where
    Some(x) => x > 0
    _ => true
```

In this example, `Some(-1)` is still excluded: it matched `Some(x)` and failed that predicate, so it does not reach `_`. The wildcard includes only values from constructors or regions that earlier patterns did not match.

Explicit `true` and `false` branches are valid. A bare constructor is sugar for including that entire constructor:

```natcode
unique type Result
    Success(number)
    Failure(string)

type Successful = Result where
    Success

// Equivalent ways to include every Success value:
type Successful2 = Result where
    Success => true

type Successful3 = Result where
    Success(_) => true
```

For a nullary constructor, `None` and `None => true` are likewise equivalent. Explicit exclusions such as `Failure => false` and catch-alls such as `_ => false` remain valid even when omission would currently produce the same set.

A restriction that removes every possible value resolves to `Empty` and is valid. `Empty where ...` also resolves to `Empty` (TYP-REF-07).

### Binding and destructuring the input

A single binder may bind the entire candidate value regardless of how many constructors the input type contains:

```natcode
(Foo | Bar) where
    value => isAllowed(value)
```

Positional destructuring without a constructor name is available only when the complete type on the left has one unambiguous constructor shape (PAR-REF-02). One exposed parameter may omit parentheses; two or more require parentheses.

```natcode
type Positive = number where
    x => x > 0

type AboveDiagonal = Position where
    (x, y) => x > y
```

If different nominal constructors are possible, positional destructuring cannot choose between them:

```natcode
// Invalid: Foo and Bar are different nominal constructors.
(Foo | Bar) where
    (x, y) => x > y

// Valid: each branch says which constructor it handles.
(Foo | Bar) where
    Foo(x, y) => x > y
    Bar(x, y) => x < y
```

Within `Option where ...`, unqualified constructor names such as `Some` and `None` resolve contextually to `Option.Some` and `Option.None` without becoming bindings in the surrounding scope (RES-REF-01). Fully qualified names remain valid. If the input type contains multiple possible constructors with the same short name, that short name is ambiguous and must be qualified.

### Where placement and precedence

`where` is a normal type operator and may appear wherever a type expression is already permitted (PAR-REF-01). The `where` keyword must start on the **same physical line** as the type or constructor it restricts (for a parenthesized expression, the line of its closing `)`); moving it to the next line is invalid. Its branch indentation can be normalized when ownership is unambiguous (LEX-IND-01). Arithmetic, prefix complement `~`, and prefix Boolean NOT `!` bind more tightly than `where`; `where` binds more tightly than the set operators `|`, `&`, and `\`. Intersection `&` and difference `\` have equal left-associative precedence, with union `|` lower. The full placement of comparisons and Boolean `&&`/`||` relative to `where` and set operators remains open (PAR-TYPE-01):

```natcode
A | B where x => predicate(x)
// means:
A | (B where x => predicate(x))
```

Parentheses select a larger input explicitly:

```natcode
(A | B) where
    A(x) => ...
    B(x) => ...
```

`~Number where x => predicate(x)` means `(~Number) where x => predicate(x)`; `~(Number where x => predicate(x))` complements the restricted Number type instead. Ordinary `where` pattern-narrowing and operand-safety rules apply to complements (TYP-REF-18). For example:

```natcode
unique type Ticket(Number)

type PositiveTickets = ~Number where
    Ticket(x) => x > 0
```

The `Ticket(x)` pattern narrows the candidate to a nominal value whose payload is Number. By contrast, `~Number where x => x > 0` is invalid because an unrestricted whole-value binder for `~Number` cannot support a numeric comparison.

Directly chaining `where` operators is rejected. Combine conditions in one restriction or name the intermediate type instead:

```natcode
// Invalid:
number where x => x > 0 where x => x < 10

// Valid:
type Positive = number where
    x => x > 0

type SmallPositive = Positive where
    x => x < 10
```

A `where` may not be written directly after an already-applied constructor type such as `Position(0, number)` or `Position(number, number)` (PAR-REF-03). Restrict an individual parameter inside the constructor application, or name the specialized type and refine that name:

```natcode
type PositiveXAxis = Position(
    0,
    number where y => y > 0
)

type XAxis = Position(0, number)
type PositiveXAxis2 = XAxis where
    y => y > 0
```

The second form sees the constructor shape exposed by `XAxis`: only its one remaining parameter. A specialized type with zero exposed parameters, such as `Position(0, 0)`, has nothing for `where` to bind and cannot be refined again with `where`. `Empty` is the explicit exception described above.

A qualified constructor symbol may be refined directly:

```natcode
type PositiveSome = Option.Some where
    x => x > 0
```

### Restrictions in nominal declarations

Restrictions in a nominal declaration may be attached to individual constructor parameters or to the whole single constructor (TYP-REF-02):

```natcode
unique type Response
    Success(number where code => code >= 200 && code < 300)
    Failure(string)
```

The inner `where` can reference only that single parameter. It cannot reference adjacent constructor parameters.

A restriction may instead apply to a constructor as a whole, allowing relationships between that constructor's parameters:

```natcode
unique type Range(number, number) where
    (start, end) => start < end
```

For algebraic types, individual constructors may have whole-constructor restrictions:

```natcode
unique type Response
    Success(number) where
        code => code >= 200 && code < 300
    Failure(string) where
        message => message != ""
```

A declaration-wide `where` on a nominal type with multiple constructors is **not supported**:

```natcode
// Invalid: a declaration-wide `where` cannot follow a multi-constructor type.
unique type Response
    Success(number)
    Failure(string)
where
    Success(code) => code >= 200 && code < 300
    Failure(message) => message != ""
```

To filter an existing algebraic nominal type as a whole, use a transparent `type` declaration instead:

```natcode
type FilteredResponse = Response where
    Success(code) => code >= 200 && code < 300
    Failure(message) => message != ""
```

`where` on a constructor declaration remains legal and can relate multiple parameters, unlike a parameter-local restriction. These restrictions do not change nominal identity. For example, restricting the payload accepted by a nominal `Port` constructor does not make `Port` assignable to `number`.

When more than one restriction applies, all of them must hold; their effect is intersection (TYP-REF-02).

### Specialized aliases and constructors

Applying a constructor in a type expression fixes any literal positions and leaves the remaining positions as the parameters exposed by the resulting specialized type (TYP-NOM-07):

```natcode
unique type Position(number, number)

type XAxis = Position(number, 0)
type Center = Position(0, 0)

x = XAxis(10)    // constructs Position(10, 0)
center = Center  // zero-parameter application
center2 = Center()
```

A zero-parameter application may omit `()` in both type and value contexts (TYP-NOM-08). Thus `x: Center` and `x: Center()` name the same type, and nullary constructors such as `Option.None` and `Option.None()` are equivalent.

A restricted alias preserves the constructor structure of what it restricts rather than inventing new constructor shortcuts (TYP-REF-04). For an algebraic type, every constructor whose restricted set is not proven `Empty` is exposed under the alias namespace:

```natcode
unique type Option
    Some(number)
    None

type PositiveOption = Option where
    Some(x) => x > 0

value = PositiveOption.Some(5)
```

`PositiveOption.None` is absent because `None` was eliminated. `PositiveOption(5)` is not introduced, because `Option` itself was never a constructor. If the developer wants a directly callable constructor alias, the constructor itself can be restricted:

```natcode
type PositiveSome = Option.Some(number where x => x > 0)
value = PositiveSome(5)
```

A constructor is removed from a restricted alias only when the compiler can prove that its remaining set is `Empty`. Lack of proof that a constructor is inhabited is not itself proof that the constructor is empty; construction still must establish the restriction for the specific value.

Construction through a restricted alias enforces every applicable restriction.

### Constructibility through set algebra

Constructibility is determined from the **normalized resulting type**, not mechanically inherited from one operand (TYP-REF-05).

If a union or intersection still exposes one common effective constructor shape, the resulting alias remains directly constructible:

```natcode
unique type Vector3(number, number, number)

type PositiveX = Vector3 where
    (x, y, z) => x > 0

type PositiveY = Vector3 where
    (x, y, z) => y > 0

type EitherPositive = PositiveX | PositiveY
type BothPositive = PositiveX & PositiveY
```

Both aliases still expose the same three `Vector3` parameter positions, with their combined restrictions.

Sharing only the same nominal origin is not enough:

```natcode
type XAxis = Position(number, 0)
type YAxis = Position(0, number)

type Axis = XAxis | YAxis
```

`Axis` is a valid type but is not directly callable: `XAxis` and `YAxis` expose different parameter positions. By contrast:

```natcode
type Center = XAxis & YAxis
```

normalizes to `Position(0, 0)`, so `Center` has the resulting zero-parameter constructor.

Set difference preserves constructibility when its normalized result still has one effective constructor shape. The type-expression operator `A \ B` denotes values in `A` that are not in `B` (TYP-UNI-10).

### Purity and lexical capture

A `where` predicate must be pure and deterministic (TYP-REF-06). Type membership cannot depend on I/O, mutable state, time, randomness, network or database state, or other effects.

A predicate may call eligible pure functions. It may also close over earlier immutable bindings in lexical scope (RES-REF-02):

```natcode
minimum = 10

type AboveMinimum = number where
    x => x > minimum
```

The captured binding is the specific earlier binding, consistent with Natcode's lexical binding model. Values needed for mandatory compile-time validation must still be evaluable or provable by the compiler; effectful or runtime-dependent state is not permitted in a type restriction.

Restricted constructors are not themselves Boolean membership tests. Compose restrictions through types instead:

```natcode
type Positive = number where
    x => x > 0

type SmallerPositive = Positive where
    x => x < 10
```

### Runtime validation

The infix operator `as?` explicitly validates membership in a restricted type at runtime (TYP-REF-12, PAR-VAL-01). It is validation, not conversion or construction: on success the result is the original value, now known as the requested target type; on failure the result is a designated `ValidationError` value.

Every runtime-validatable restricted target `T` has a **validation input type** `U`: the compatible unrestricted type whose values can safely be checked by `T`'s restrictions. For example, the validation input type of `number where x => x > 0` is `number`, and the validation input type of `Value("one" | "two")` is `Value`. A source expression in `x as? T` must already be statically assignable to `U`; `as?` does not perform dynamic discrimination between unrelated underlying types. Use `when` for that. A target whose validation input type is `Empty` is invalid for `as?`, because no value can be supplied to such a validator.

```natcode
type Positive = number where
    x => x > 0

n: number = getNumber()
validated = n as? Positive       // Positive | ValidationError

text: string = getString()
invalid = text as? Positive      // compile error: string is not a number
```

Arithmetic binds before `as?` validation. To apply validation *inside* an arithmetic expression, parentheses are required: `a / (b as? NonZero)` validates `b` first, whereas `a / b as? NonZero` attempts division first and then validates that result (which requires the divisor already be proved nonzero). The right operand of `as?` is a complete type expression. Thus:

```natcode
x as? number where n => n > 0
```

means:

```natcode
x as? (number where n => n > 0)
```

For every valid application, the result type is stably `T | ValidationError` (TYP-REF-13). This signature does not collapse merely because the compiler can prove that the particular validation will succeed or fail. A proof may remove runtime work or precompute the result, but it does not change the expression's type. Whether such statically decided validations should produce a diagnostic remains open (TYP-DIAG-03).

Runtime validation applies to any runtime-checkable restricted subset with one compatible non-empty validation input type, not only a subset written directly with `where` (TYP-REF-14). This includes constructor-payload restrictions and compatible unions, intersections, and differences. Sibling refinements of the same validation input type may validate against one another. A refinement built on another refinement validates the full inherited restriction chain.

For a union target, validation succeeds when any alternative succeeds. Failures from alternatives are discarded when the union succeeds; if every alternative fails, the resulting `ValidationError` captures the failures from all determinable alternatives. For an intersection target, every component must succeed and all determinable component failures are collected. Set-difference validation follows ordinary membership semantics: the value must belong to the left side and not the right side (TYP-REF-14, TYP-REF-15).

`ValidationError` is a first-class designated error type that carries the type-level validation failures encountered while checking the requested target. Validation evaluates all applicable checks needed to collect every determinable failure rather than stopping after the first independent failure (TYP-REF-15). The exact structure of `ValidationError`, developer-defined type-level validation messages or metadata, and their representation remain open (TYP-REF-16).

A target type therefore conceptually induces a validator with the shape `U -> T | ValidationError`. General expression-placeholder syntax such as `_ as? T`, including multiple-placeholder and scope rules, remains open (PAR-EXPR-02). Pipe syntax and any error-aware short-circuiting behavior also remain open (PAR-PIPE-01, TYP-REF-17).

## Type-expression computations and record syntax

Type expressions accept named and qualified types, constructor restrictions, parentheses, primitive literal types (numeric literals including negative values and decimals; double-quoted strings; and `true`/`false`), and structural record types (PAR-TYPE-09). Record types use `{}` for the empty record, `key: Type` properties, required commas between properties, and an optional trailing comma. Static keys can be identifiers, string literals, or numeric literals, matching object literals. Computed and dynamic property typing remains an open question (TYP-OBJ-01).

```natcode
type User = { name: String, age: Number, active: Boolean }
type Tagged = { kind: "left", value: Number }
type EmptyRecord = {}
```

Type expressions may perform numeric arithmetic (`+`, `-`, `*`, `/`, `%`), comparisons, and Boolean operations (`&&`, `||`, `!`) on the possible values of their operand types (PAR-TYPE-10, TYP-COMP-01). A not-yet-spelled Euclidean modulo operator remains separately open (PAR-EXPR-03). Arithmetic operates on **sets of possible results**: for example, `10 * (2 | 30)` describes `20 | 300`, and `(1 | 2) + (10 | 20)` describes `11 | 12 | 21 | 22`. Finite literal combinations are evaluated exactly. Expressions over infinite or refined numeric sets may retain symbolic constraints and simplify them when equivalence is established, subject to the still-open numeric representation and rounding rules (TYP-BLT-05/06). Division and remainder retain the required nonzero-divisor proof (TYP-ARITH-04).

Comparisons produce possible Boolean literal results, and Boolean operations obey the same **strict Boolean** rule at type level as at runtime (TYP-BOOL-01). Invalid operand possibilities are compile errors, not filtered out:

```natcode
type Answer = 40 + 2              // 42
type Products = 10 * (2 | 30)    // 20 | 300
type Compare = (1 | 2 | 3) > 2   // Boolean
type IsTrue = (10 > 5) && true  // true
type Invalid = (true | 42) && true // compile error
```

A type computation can refer to a value binding through that binding's inferred type. The binding need not have one known runtime value. All occurrences of the **same** binding denote the same underlying choice, and correlated values derived through intermediate bindings remain dependent; independently written unions denote independent sets of possible values (TYP-COMP-02).

```natcode
minimum: Number = 10
type Threshold = minimum + 2 // 12

x = when getBoolean()
    true => 1
    false => 2
y = x + 1

type Related = x + y          // 3 | 5
type Independent = (1 | 2) + (1 | 2) // 2 | 3 | 4
```

This does not authorize arbitrary effectful type computation. Pure user-defined compile-time evaluation guarantees and resource budgets remain TYP-EVAL-02; explicit generic declarations and advanced type-level functions remain TYP-GEN-01.

The approved type-expression precedence is: grouping and constructor application; prefix `~`, `!`, and unary `-`; multiplicative arithmetic; additive arithmetic; `where`; `&` and `\` (equal, left-associative); then `|`. Thus arithmetic and prefix operators bind more tightly than `where`, which binds more tightly than set operators. The placement of comparisons and Boolean `&&`/`||` relative to `where` and set operators has **not yet** been approved (PAR-TYPE-01), so parentheses should be used where those forms mix. No explicit `<T>` generic grammar has been adopted (TYP-GEN-01).


## Union and intersection types

- `|` is mathematical **set union**: a value belongs to `A | B` precisely when it belongs to `A` or `B`. Neither union nor intersection creates a new nominal identity (TYP-UNI-06).
- Simplification/consolidation of a union is permitted **only when it preserves the exact represented set of possibilities**. In particular, preserve correlations between fields and exclusions; do not merge distinct object variants in ways that admit combinations not present in the original union.
- `&` is mathematical **set intersection**: a value belongs to `A & B` precisely when it belongs to **both** types. It never constructs or combines constituent values and is distinct from `:` constraints (TYP-UNI-06).
- `\` is mathematical **set difference** in type expressions: `A \ B` contains exactly the values in `A` that are not in `B` (TYP-UNI-10).
- Prefix `~` is **set complement** relative to `Any`: `~A = Any \ A` (TYP-UNI-05). This is distinct from Boolean NOT `!`.
- `Any` is the universal type (`T | Any = Any`, `T & Any = T`). `Empty` denotes the empty type: no values inhabit it. Thus `A | Empty = A` and `A & Empty = Empty` (TYP-UNI-07).
- An intersection proven empty resolves to `Empty` and is **valid**, whether direct, nested, or involving explicit `Empty`. The declaration itself causes no warning or error merely for being empty. Attempting to provide a value where the type is uninhabited is an error (TYP-UNI-08).
- Independently declared, unrelated nominal constructor types are disjoint: for example, `UserId & ProductId` is `Empty` (TYP-UNI-09). Nominal subtyping or other explicit overlapping-identity relationships are not introduced by ordinary constructors.

For example, this union:

```natcode
{ kind: "left", value: number }
|
{ kind: "right", value: string }
```

must not be simplified into:

```natcode
{ kind: "left" | "right", value: number | string }
```

because the latter would allow a `"left"` tag paired with a `string` value, which the original type excludes. The record-type syntax is approved by PAR-TYPE-09.

Set identities include `A | B = B | A`, `A | A = A`, `A & B = B & A`, and `A & A = A`. More detailed structural membership and normalization choices remain TYP-UNI-04. Difference `\` is TYP-UNI-10 and complement `~` is TYP-UNI-05. Explicit generic declarations remain open (TYP-GEN-01); broader subtyping remains TYP-CON-03.

## Target directions

Natcode is intended to support a native interpreter and compilation to JavaScript, WebAssembly, C, and potentially Python (PY-TGT-01). These are project goals rather than language semantics. A shared compiler IR/runtime design and the implementation language are TOOL-ARCH-01.
