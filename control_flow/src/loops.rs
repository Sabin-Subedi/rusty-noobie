pub fn loops() {
    // rust has 3 kind of loops: for while and loop

    println!("I am inside loops");

    // loop

    // loop {
    //     println!("again!"); this creates a infinite loop
    // }

    // returning value from a loop
    let mut n = 0;
    let result = loop {
        n += 1;
        if n > 10 {
            break n * 2;
        }
    };

    println!("The result is {result}");

    // Loop Labels to Disambiguate Between Multiple Loops

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // While loop
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for loop
    let a = [10, 20, 30];
    for i in a {
        println!("the value is {i}");
    }
}
