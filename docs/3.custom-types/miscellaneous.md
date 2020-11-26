# Miscellaneous

## Copy, Clone [rust - trait]
Under the hood, both a copy and a move can result in bits being copied in memory, although this is sometimes optimized away.

There is a small difference between the two: the derive strategy will also place a Copy bound on type parameters, which isn't always desired.

https://doc.rust-lang.org/core/marker/trait.Copy.