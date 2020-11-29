fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // Try changing the values in the struct to see what happens
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        // you can destructure structs and rename the variables,
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
        
        // the order is not important
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y),
    }
}
