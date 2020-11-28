#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

impl Copy for Point {}

impl Clone for Point {
    fn clone (&self) -> Point {
        *self
    }
}
// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

impl Copy for Rectangle {}

impl Clone for Rectangle {
    fn clone (&self) -> Rectangle {
        *self
    }
}

fn rect_area (points: Rectangle) -> f32 {
    let Rectangle { 
        top_left,
        bottom_right,
    } = points;

    let width: f32 =  bottom_right.x - top_left.x;
    let height: f32 = bottom_right.y - top_left.y;

    width * height
}

fn square (points: Rectangle) -> f32 {
    let Rectangle { top_left, bottom_right } = points;
    let width = bottom_right.x - top_left.x;
    let height = bottom_right.y - top_left.y;
    
    if height > width {
        width.powf(2.0)
    } else {
        height.powf(2.0)
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);


    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.3, ..point };
    // [JS lingo] spread the point variable
    // let bottom_right = Point { ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: top_edge, y: left_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // Exercise 1)

    let rect = Rectangle {
        top_left: Point { x: 0.0, y: 0.0 },
        bottom_right: Point { x: 3.0, y: 4.0 },
    };

    println!("Area of 'rect' is {} unitless", rect_area(rect));

    // Exercise 2)
    println!("Area of 'rect' becoming a square is {} unitless", square(rect))
}
