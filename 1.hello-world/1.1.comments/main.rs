// Same as javascript, with the addition of
// Double slashes are the style to stick to

/// Triple slashes are doc commments, for the following item
/// When running `rustdoc` these comments get compiled in documentation
/// Supported by markdown


/* This block comments can be used in expressions */

fn main () {
//! This comments, for the enclosing item, i.e. inside function
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100, x = {}", x)
}