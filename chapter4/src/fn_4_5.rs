use thiserror::Error;

pub fn fn_4_5() {
    fn_ex_print_result(fn_ex_div_result(10, 5));
    fn_ex_print_result(fn_ex_div_result(10, 0));
    print_mydiv(5, 2);
    print_repeat_mydiv(repeat_mydiv(&[(2, 1), (-9, -3), (5, 2)]));
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

#[derive(Error, Debug)]
enum DivError {
    #[error("[0] divided by zero")]
    DivByZero(i32), // 0で割り算、i32は分子
    #[error("Both numerator [0] and denominator [1] are negative")]
    BothNegative(i32, i32), // 分子、分母ともに負の数、2つのi32はそれぞれ分子、分母
}

fn mydiv(x: i32, y: i32) -> Result<i32, DivError> {
    if y == 0 {
        Err(DivError::DivByZero(x))
    } else if x < 0 && y < 0 {
        Err(DivError::BothNegative(x, y))
    } else {
        Ok(x / y)
    }
}

fn print_mydiv(x: i32, y: i32) {
    match mydiv(x, y) {
        Ok(ans) => println!("no error. ans = {}", ans),
        Err(e) => println!("{}", e),
    }
}


fn repeat_mydiv(aray: &[(i32, i32)]) -> Result<Vec<i32>, DivError> {
    let mut ret = Vec::new();
    for aa in aray {
        let ans = mydiv(aa.0, aa.1)?;
        ret.push(ans);
        println!("pushed: {} / {} = {}", aa.0, aa.1, ans);
    }
    Ok(ret)
}

fn print_repeat_mydiv(result: Result<Vec<i32>, DivError>) {
    match result {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("print_repeat_mydiv: {}", e),
    }
}
