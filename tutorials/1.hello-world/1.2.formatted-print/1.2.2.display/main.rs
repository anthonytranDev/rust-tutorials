// Display is used to format your console messages to be more compact and clean

use std::fmt;

// Define a structure for which `fmt::Display` will be implemented. This is
// a tuple struct named `Structure` that contains an `i32`.

// Define a structure for which `fmt::Display` will be implemented. This is
// a tuple struct named `Structure` that contains an `i32`
struct Structure(i32);

#[derive(Debug)]
struct MinMax(i64, i64);

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64
}

#[derive(Debug)]
struct Complex {
    real: f32, imag:f32
}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
// No ideal style for all types and the std library doesn't presume to
// dictate one.
// For any new container type which is not generic,fmt::Display can be
// implemented.
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // Write strictly the first element into the supplied output
    // stream: `f`.
    
    // Returns `fmt::Result` which indicates whether the operation 
    // succeeded or failed. Note that `write!` uses syntax which
    // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Similarly, implement `Display` for `Point2D`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}


fn main () {
    let minmax = MinMax(0, 14);
    println!("Compare structures!");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let point = Point2D {x: 3.3, y: 7.2};
    println!("Compare points!");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let imaginary_point = Complex { real: 3.3, imag: 7.2 };
    println!("Display: {}", imaginary_point);
    println!("Debug: {:?}", imaginary_point);
}