# jjay

**jjay** is a toy JSON templating language.

## Features

- [x] Compatible with existing JSON data.
- [x] Single-line comments (`//`) and block comments (`/* ... */`)
- [x] Variables
- [x] Functions
- [x] Numeric operators (`+`, `-`, `*` and `/`)
- [ ] Equality operators (`==`, `!=`, `<`, `<=`, `>`, `>=`) **[not implemented]**
- [ ] Path access (`var.x.y`, `var["x"].y`) **[not implemented]**
- [ ] Standard library of functions **[not implemented]**

## Syntax

A jjay script consists of zero or more statements, followed by a single expression.
Every statement ends with `;`, while the final expression does not.

### Expressions

Expressions are pieces of jjay code that evaluate to a value.

Expressions can be any JSON-like value, such as objects, arrays, strings, numbers, booleans and
`null`. They may also be variables (`x`) or function calls (`f(expr)`), or they may be blocks
containing more statements and expressions (`(...)`).

### Objects

Objects are written similarly to pure JSON syntax, but object keys may be unquoted if they are valid
identifiers. Trailing commas are also allowed.

```
{
  x: 3,
  "y": 4,
}
```

### Arrays

Arrays are written similarly to pure JSON syntax, but trailing commas are allowed:

```
[ 1, 2, 3, ]
```

### Other JSON value types

Strings, numbers, booleans and `null` are identical to their pure JSON equivalents.

### Variable declarations

Variables are declared with the `let` statement:

```
let x = 3;
let y = 4;
[ x, y ] // = [ 3, 4 ]
```

Variables may not have the same name as a previously declared variable, except:
* Function parameter may have the same names as variables in the scope the functon is declared.
* Variables declared inside of a block may have the same name as a variable in the scope outside the
  block.

### Function declarations

Functions are declared with the `let` statement, with one or more argument group:

```
let f(x) = x + 1;
let g(x)(y) = x + x * y;
f(1) + g(2)(3) // = 10
```

Lambda functions are not supported, but functions can be declared inside of blocks:

```
let apply(func)(arg) = func(arg);
(let f(x) = x * x; f)(4) // = 16
```

A function inherits the scope outside it, but variables within it may shadow variables in the outer
scope.

```
let x = 3;
let f(x) = x + 3;
f(5) // = 8
```

### Operators

Operators are implemented as built-in functions.

jjay has the following arithmetic operators:

* `+`: Add numbers, or concatenate strings or arrays.
* `-`: Subtract numbers.
* `*`: Multiply numbers.
* `/`: Divide numbers.

jjay has the following comparison operators:

* `==`: Check if both sides are equal.
* `!=`: Check if both sides are different, evaluates to boolean.
* `>`: Check if left-hand operand is less than right-hand operand.
* `>=`: Check if left-hand operand is less than or equal to right-hand operand.
* `<`: Check if left-hand operand is greater than right-hand operand.
* `<=`: Check if left-hand operand is greater than or equal to right-hand operand.

> **Note:** The comparison operators are understood by jjay, but their associated functions have
  not been implemented.

Numbers are compared numerically. Strings and arrays are compared lexiographically. `true` is
greater than `false`. Values of different types may not be compared, except for equality or
inequality.

jjay also has a "pipeline" operator, `|`. The right-hand side is invoked as a function with the
left-hand side as the argument, so `x | f` is equivalent to `f(x)`.

| Operator | Function name | Precedence | Associativity |
|----------|---------------|------------|---------------|
| `|`      | `/pipe`       | 0          | Left-to-right |
| `==`     | `/eq`         | 1          | Left-to-right |
| `!=`     | `/ne`         | 1          | Left-to-right |
| `<=`     | `/le`         | 1          | Left-to-right |
| `>=`     | `/ge`         | 1          | Left-to-right |
| `<`      | `/lt`         | 1          | Left-to-right |
| `>`      | `/gt`         | 1          | Left-to-right |
| `+`      | `/add`        | 2          | Left-to-right |
| `-`      | `/sub`        | 2          | Left-to-right |
| `*`      | `/mul`        | 3          | Left-to-right |
| `/`      | `/div`        | 3          | Left-to-right |

### Blocks

A block is zero or more statements, followed by a single expression. While similar to a script,
a block must always be surrounded by parentheses.

A block inherits the scope outside it, but variables within it may shadow variables in the outer
scope.

### Built-in variables and functions

* `scope()`: Return an object with all variables in the current scope and all outer scopes.
* `local_scope()`: Return an object with all variables in only the current scope.
