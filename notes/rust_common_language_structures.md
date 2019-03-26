# Common Concepts in Rust

### Identifiers

Two kinds:

1. first character is a letter & the rest are alphanumeric or `_`
2. first character is `_` and the raminder are alphanumeric or `_` and `len(<identifier>) > 0`

You can create __raw identifiers__ (which are identifiers for reserved words) by typing `r#` in front of it. Ex:

```rust
let r#fn = "this is a string named 'fn'";
```

### Variables

Variables are by default immutable.

```rust
let x = 5; // this is immutable

let mut y = 6; // this is mutable
```

### Constants

Constants are actually constants, rather than just immutable variables.

They have to be calculated at compile time. 

```rust
const MAX_POINTS: u32 = 100_000;
```



### Shadowing

By reusing the `let` keyword, you can change the reference of a variable within a given scope, but not changing the old value. When you shadow a variable, it can have a different type.

```rust
let spaces = "   ";
let spaces = spaces.len();
```



## Data Types

Types are typically inferred, but not always. If it's ambiguous, you will have to give a value a type annotation.

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

### Scalar Types

#### Integers

You can have 8, 16, 32, 64, and 128 bit integers, signed or unsigned as `i<num_bits>` and `u<num_bits>` respectively. There's also the `isize` and `usize` integers, which uses the standard size from the computer architecture. `i32` is the default, and use something else only when necessary. 

If you want to explicitly rely on integer overflow, use the `Wrapping` type in the standard library.

#### Floating Point

Both `f64` and `f32` types exist as equivalents to `double` and `float` primitives from Java and are declared the same way.

#### Boolean

booleans are exactly as you would expect and are 1 byte in size.

#### Character

Characters in Rust are __Unicode__. _Interesting_...

### Compound Types

#### Tuples! 

Tuples appear to be treated like they are in OCaml. Makes sense, since Rust is technically ML-based. You can unwrap them intelligently! Also, they're mixed type :clap:

```rust
let tup = (500, 6.4, 1);

let (x, y, z) = tup;

let x2 = tup.0
```

#### Arrays

They all have to be the same type, and have an assigned length (as is standard). The type declaration is `[type; number]`, where `type` is the type of the values and `number` is the length. They're accessed using `[]`



### Functions

Everything in Rust use snake case.

function declarations are done using the `fn` keyword.

Variables can only be assigned expressions. Expressions are 

1. Constants & other variables
2. Function calls
3. Macro calls
4. Blocks (code inside of `{ }` blocks)

Functions implicitly return the last value in the function. This must be an expression, rather than a statement. Don't put a semicolon if you want it to stay an expression

### Control Flow

```rust
if <boolean expression> {
    // do something
} else if <other boolean expression> {
    
} else {
    // do something different
}
```

You can use `if` expressions in a `let` statement, but the types have to be the same.



For loops, there's 

- `loop`, which is basically a `while(true)`
- `while`
- `for` (which does a for each?)