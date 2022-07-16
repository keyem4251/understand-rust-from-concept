pub fn section7_2() {
    println!("{}", fact(3));
}

fn fact(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * fact(n - 1)
    }
}
