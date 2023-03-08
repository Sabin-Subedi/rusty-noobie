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
    println!("The value of x after new declaration is: {x}");

    //  data types in rust
    /*
    1. Scalar Data types
        a. Integer
            i for signed and u for unsigned
            range for signed -> -2^n-1 to (2^n-1)-1 eg: for 8bit -> -128 to 127
            range for unsinged -> 0 to 2^n -1 eg: for 8 bit -> 0 to 255

        b. Floating-point
            has two types f32 with single precision and f64 for double precision
            f64 is default float

        c. Boolean Types
            True and false
            Mostly useful for the control flow of the applcation

        d. Character Types
            primtive alphabet data type
            specified by a single quote ''
            4 bytes in size and represents  a unicode scalar value
     */

    //Integers

    // unsinged integer
    let x: u32 = 23;
    // signed integer
    let z: i32 = -23;
    // assign unsinged integer of size that depends on the architecture of your computer
    // for 8085 microprocessor it's u8
    // for 64bit arch it's u64
    let custom_size: usize = 23399;
    println!("this is integers : signed -> {z} unsigned -> {x} custom -> {custom_size}");

    // float Number
    let a = 2.0; // f64
    let b: f32 = 3.0; // f32
    println!("this is floating number: f64 -> {a} f32 -> {b}");

    // char type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    println!("This is a character: {c} {z}");

    /*
       2. Compound Data type
           it can group multiple values into one type.

           a. Tuple type
               it has fixed length: once declared never grow or shrink
               can have multiple data types in one tuple
               can access value using the . with the index or desctructuring

           b. Array
               it is collection of multiple values of same data types
               array in rust has fixed length
               Arrays are useful when you want your data allocated on the stack rather than the heap
    */

    //tuple
    let d = (1, 0.4, 'c');
    let (i, j, k) = d;
    println!("This is a value from tuple d: {i} {j} {k}");

    // array
    let e: [i32; 5] = [1, 2, 3, 4, 5];
    let first_element: i32 = e[0];
    println!("this is data from array e: {first_element}");
}
