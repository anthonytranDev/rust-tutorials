fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    // Inclusive range for 1, exclusive for 101
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
