pub fn fn_4_5() {
    fn_ex_print_result(fn_ex_div_result(10, 5));
    fn_ex_print_result(fn_ex_div_result(10, 0));
}

fn fn_ex_div_result(x: i32, y: i32) -> Result<i32, &'static str> {
    if y == 0 {
        Err("div by zero")
    } else {
        Ok(x / y)
    }
}

fn fn_ex_print_result<T: std::fmt::Display, E: std::fmt::Display>(ans: Result<T, E>) {
    match ans {
        Ok(res) => println!("{}", res),
        Err(str) => println!("{}", str),
    }
}
