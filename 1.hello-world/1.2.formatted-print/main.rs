fn main () {
    // println!
    // prints to newline on io::stdout
    println!("This will {}", "print");
    println!("each part on a new line");
    println!("");

    // format!
    // Creates a String using interpolation of runtime expressions
    // Argument 1 - format string, must a string literal
    // The power of the formatting string is in the {}s contained.
    println!("{}", format!("Hello {}", "World!"));
    println!("{}", format!("Counting from {smallest} to {largest}", smallest = 0, largest = 100));
    
    // print!
    // Same as format but for same line
    // prints to io::stdout
    print!("Hello {}", "World!");
    print!("Counting from {smallest} to {largest}", smallest = 0, largest = 100);
    
    // eprint!
    // like print!, but prints to io::stderr
    eprint!("Hello {}", 100);
    eprint!("Counting from {smallest} to {largest}", smallest = 0, largest = 100);
}