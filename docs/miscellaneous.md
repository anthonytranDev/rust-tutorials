# Miscellaneous

## Styling

## `::` syntax

Prefer to include use the `::` after a trait. To distringuish and give code better context like `fmt::Result` (instead of `std::fmt::Result`) and `std::Result` 

## Different indentations

- 4 Indentations by default
- 3 Indentations by conditional statements
- 1 Indentations used for styling and giving things space

## Destruction from either side (it depends)
You can destructure on left side of the assignment
```rust
struct Point {
    x: i32
    y: i32
}

// Treat like JavaScript object x and y can be in any order
const { x, y } = Point { y:1, x:2 };
```

You can destructure on right side of the assignment for conditional
```rust
let amount: Option<i32> = None

if let Some(i) = amount {
    println!(amount > 10);
}
```

## Circumstances where you do need to use the return keyword

```rust
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Corner case, early return
    if rhs == 0 {
        return false;
    }

    // This is an expression, the `return` keyword is not necessary here
    lhs % rhs == 0
}
```

## Questions

### What are theses syntaxes?

- `::`
- `..`

