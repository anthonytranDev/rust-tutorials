# Glossary

Headings for each section are complete, ones that are empty are kept that way, waiting to filled out more in the future.

These words originate from the following;
1. Sections under study
2. Files under study
3. Error messages after post compilation, of files
4. Words found in documents, that relation to the above topics

## Questions/Unknowns

Any variant which is valid as a struct is also valid as an enum.?

## struct (keyword)
Three types of structures `structs`
- Tuple structs
- classic C structs
- Unit structs, which are field-less, are useful for generics

https://doc.rust-lang.org/rust-by-example/custom_types/structs.html

## linked lists (computer science - data structure)
1. starts with a node Head which is the start of the linked list, with zero _next link_ pointing to it
2. this node head points to another single node. Each and every node contains contains data and next link pointing to another consecutive node.
3. The last node's next link will point to `null`

https://www.tutorialspoint.com/data_structures_algorithms/linked_lists_algorithm.htm

### Enumerate
To mention separately as if in counting; name one by one

## static
`static` is a reserved lifetime name

Data pointed to by the reference lives for the entire lifetime of the running program.

Reference lives -> Lifetime

Variables
- can be made with 
  - `'static` lifetime OR
  - const `static` declaration

### Reference Lifetime
Attempt to understand lifetime(s).

As a reference lifetime 'static indicates that the data pointed to by the reference lives for the entire lifetime of the running program. It can still be coerced to a shorter lifetime.

## References
struct - https://doc.rust-lang.org/rust-by-example/custom_types/structs.html