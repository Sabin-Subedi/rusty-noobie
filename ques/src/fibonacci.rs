pub fn gen_fibo(num: i32) {
    let mut counter = 0;
    let mut first_num = 1;
    let mut second_num = 1;

    while counter < num - 2 {
        if counter == 0 {
            println!("{first_num}\n{second_num}")
        }
        let third_num = first_num + second_num;
        first_num = second_num;
        second_num = third_num;
        println!("{third_num}");
        counter += 1;
    }
}
