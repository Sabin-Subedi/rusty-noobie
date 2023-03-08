fn main() {
    println!("Hello, world!");
    let number = 3;
    // if expressions
    if number < 5 {
        println!("Less than 5");
    } else {
        println!("Greater than 5");
    }

    // else if condition
    if number % 2 == 0 {
        println!("Even number");
    } else if number == 0 {
        println!("Zero");
    } else {
        println!("Odd number");
    }

    // use if in a let statement
    let number = if number <= 3 { 6 } else { 3 };

    println!("The value of number is: {number}");
}
