# Glossary

## Things to consider
If they are English words remember to include both a/an;

 - English definition
 - Computer science definition
 - Rust definition

Remember to include a reference, vague, accurate enough to be searchable.

If they are particular syntax or command related symbol/reserve;

  - Get the best most expansive source for it
  - For example `cargo` should point to the Rust Cargo handbook

## bitwise operation
are used for manipulating data at the bit level, also called bit level programming - https://www.guru99.com/c-bitwise-operators.html 

## std::mem::size_of
Returns the size of type in bytes

## tuples
Collection of different types. Each value in tuple has a type signature `(T1, T2, ...)`. Where `T1` and `T2` are its members

## array
An array is a collection of all the same type `T`, stored in contiguous memory. There length is known at compile time.

There type signature is `[T; length]`

### contigious (English)
Definition: Sharing an edge or boundary

### slice
Slices point to a section of the array, are formed via ` [starting_index..ending_index]`

Where `ending_index` is one more than the last position in the slice