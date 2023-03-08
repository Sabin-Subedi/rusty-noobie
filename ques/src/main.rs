mod fahrenhiet_to_celcius;
mod fibonacci;

fn main() {
    println!("Hello, world!");
    let fahrenhiet = 58;
    let celcius: f64 = fahrenhiet_to_celcius::convert_fahrenhiet_to_celcius(fahrenhiet);

    println!("Fahrenhiet: {fahrenhiet} ----> Celcius: {celcius}");

    println!("Generating upto  5th fibonacci number");
    fibonacci::gen_fibo(5);
}
