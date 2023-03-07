fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    // x=6; This throws an error as variable in  rust is immutable by default.
    let mut y = 7; // mut helps to make a variable mutable so that we can reassign a value to a variable.
    println!("The value of y is: {y}");
    y = 3;
    println!("The value of y is: {y}");

    // Constants
    // Constants should be uppercase separated with _ in rust
    const TIME: u32 = 60 * 60;
    println!("The value of a constant is: {TIME}");

    // Variable Shadowing
    let x = 6;
    println!("The value of x after new declaration is: {x}")
}
