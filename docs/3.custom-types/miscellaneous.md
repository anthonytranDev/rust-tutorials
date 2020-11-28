# Miscellaneous

## Copy, Clone [rust - trait]
Under the hood, both a copy and a move can result in bits being copied in memory, although this is sometimes optimized away.

There is a small difference between the two: the derive strategy will also place a Copy bound on type parameters, which isn't always desired.

https://doc.rust-lang.org/core/marker/trait.Copy.

## box (rust - type)
All values in Rust are stack allocated by default. Values can be boxed (allocated on the heap) by creating a Box<T>. A box is a smart pointer to a heap allocated value of type T.

https://doc.rust-lang.org/rust-by-example/std/box.html?highlight=Box#box-stack-and-heap

## heap (computer science - data structure)


## use crate:: and use std::
Conjecture. I think that both `crate` and `std` are both in the global scope