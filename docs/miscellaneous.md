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

## Questions

### What are theses syntaxes?

- `::`
- `..`

