# Rue Language Specification

## 1. Introduction

Rue is a minimal programming language with a Rust-like syntax. All values are 64-bit signed integers. Programs are compiled to native executables that return their result as the process exit code.

## 2. Lexical Structure

### 2.1 Character Set
Rue source code is UTF-8 encoded text.

### 2.2 Tokens

#### 2.2.1 Keywords
```
fn let if else while
```

#### 2.2.2 Identifiers
An identifier is a sequence of letters, digits, and underscores that does not start with a digit and is not a keyword.

```
identifier ::= (letter | '_') (letter | digit | '_')*
letter     ::= 'a'..'z' | 'A'..'Z'
digit      ::= '0'..'9'
```

#### 2.2.3 Literals
Integer literals are sequences of decimal digits.

```
integer_literal ::= digit+
```

#### 2.2.4 Operators
```
+ - * / % <= >= < > == != =
```

#### 2.2.5 Delimiters
```
( ) { } ,
```

#### 2.2.6 Whitespace
Whitespace consists of spaces, tabs, and newlines. Whitespace is ignored except as a token separator.

#### 2.2.7 Comments
Currently, Rue does not support comments.

## 3. Syntax

### 3.1 Grammar
The following grammar is presented in EBNF notation:

```ebnf
program ::= function*

function ::= "fn" identifier "(" parameter? ")" block

parameter ::= identifier

block ::= "{" statement* expression? "}"

statement ::= let_statement | assignment_statement | expression_statement

let_statement ::= "let" identifier "=" expression

assignment_statement ::= identifier "=" expression

expression_statement ::= expression

expression ::= if_expression | while_expression | binary_expression | call_expression | primary_expression

if_expression ::= "if" expression block ("else" block)?

while_expression ::= "while" expression block

binary_expression ::= expression binary_operator expression

call_expression ::= identifier "(" expression? ")"

primary_expression ::= identifier | integer_literal | "(" expression ")"

binary_operator ::= "+" | "-" | "*" | "/" | "%" | "<=" | ">=" | "<" | ">" | "==" | "!="
```

### 3.2 Operator Precedence
Operators are listed from highest to lowest precedence:

1. Function calls: `f(x)`
2. Multiplicative: `*`, `/`, `%`
3. Additive: `+`, `-`
4. Comparison: `<=`, `>=`, `<`, `>`, `==`, `!=`

Operators of the same precedence are left-associative.

## 4. Static Semantics

### 4.1 Scoping Rules
- Function parameters are scoped to their function body
- Variables declared with `let` are scoped to the block in which they are declared
- Functions are globally scoped
- Variable shadowing is not permitted within the same scope

### 4.2 Name Resolution
- All identifiers must be declared before use
- Function calls must reference declared functions
- Variable references must reference declared variables or parameters

### 4.3 Type System
- All values are 64-bit signed integers (`i64`)
- No explicit type annotations are required or permitted
- All expressions evaluate to `i64`

## 5. Dynamic Semantics

### 5.1 Program Execution
- Program execution begins with a call to the `main` function
- The `main` function must be defined and take either zero or one parameter
- The value returned by `main` becomes the process exit code

### 5.2 Expression Evaluation

#### 5.2.1 Literals
Integer literals evaluate to their numeric value.

#### 5.2.2 Variables
Variable references evaluate to the current value of the variable.

#### 5.2.3 Binary Operations
Binary operations are evaluated left-to-right according to precedence:

- `+`: Addition (wrapping on overflow)
- `-`: Subtraction (wrapping on overflow)  
- `*`: Multiplication (wrapping on overflow)
- `/`: Division (program aborts on division by zero)
- `%`: Modulo (program aborts on division by zero)
- `<=`, `>=`, `<`, `>`: Comparison (returns 1 for true, 0 for false)
- `==`, `!=`: Equality (returns 1 for true, 0 for false)

#### 5.2.4 Function Calls
Function calls:
1. Evaluate the argument expression (if present)
2. Create a new scope for the function body
3. Bind the parameter (if present) to the argument value
4. Execute the function body
5. Return the value of the final expression

#### 5.2.5 Conditional Expressions
`if` expressions:
1. Evaluate the condition expression
2. If the condition is non-zero, execute the `then` block
3. If the condition is zero and an `else` block exists, execute the `else` block
4. Return the value of the executed block, or 0 if no block was executed

#### 5.2.6 While Loops
`while` expressions:
1. Evaluate the condition expression
2. If the condition is zero, return 0
3. If the condition is non-zero, execute the loop body and repeat from step 1
4. The loop body value is discarded; the loop always returns 0

### 5.3 Statements

#### 5.3.1 Variable Declaration
`let` statements declare a new variable in the current scope and initialize it with the value of the expression.

#### 5.3.2 Assignment
Assignment statements update the value of an existing variable. The variable must be previously declared in an accessible scope.

#### 5.3.3 Expression Statements
Expression statements evaluate an expression and discard the result.

### 5.4 Blocks
Blocks execute their statements in order, then evaluate their final expression (if present). The block's value is the value of the final expression, or 0 if there is no final expression.

## 6. Standard Library

### 6.1 Built-in Functions
Currently, Rue has no built-in functions.

### 6.2 Runtime Behavior
- Integer overflow wraps using two's complement arithmetic
- Division by zero causes program termination
- All memory management is handled by the runtime

## 7. Examples

### 7.1 Hello World (Return 42)
```rue
fn main() {
    42
}
```

### 7.2 Factorial Function
```rue
fn factorial(n) {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    factorial(5)
}
```

### 7.3 Variable Assignment
```rue
fn main() {
    let x = 42
    x = x + 58
    x
}
```

### 7.4 While Loop
```rue
fn countdown(n) {
    while n > 0 {
        n = n - 1
    }
    n
}

fn main() {
    countdown(10)
}
```