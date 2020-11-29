fn main() {
    // variable binding - i.e. statement
    let x = 5u32;

    // Blocks are expressions too
    // so they can be used as values in assignments.
    // The last expression in the block will be assigned to the place expression
    // Blocks where the last expression ends with `;` returns `()`
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
