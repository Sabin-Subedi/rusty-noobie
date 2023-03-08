fn main() {
    println!("Hello, world!");
    another_function(12, 'm');
}

fn another_function(x: i32, unit_label: char) -> i32 {
    println!("This is another_function. It is called with paramater : x --> {x} ,  unit_label --> {unit_label}");

    x + 1
}
