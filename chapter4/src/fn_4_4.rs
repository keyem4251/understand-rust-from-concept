use std::cmp::Ordering;

pub fn fn_4_4() {
    enum Sign {
        Positive,
        Zero,
        Negative,
    }

    fn determin_sign(x: i32) -> Sign {
        match x.cmp(&0) {
            Ordering::Greater => Sign::Positive,
            Ordering::Less => Sign::Negative,
            Ordering::Equal => Sign::Zero,
        }
    }

    fn print_sign(s: Sign) {
        match s {
            Sign::Positive => println!("+"),
            Sign::Zero => println!("0"),
            Sign::Negative => println!("-"),
        }
    }

    print_sign(determin_sign(1));
}