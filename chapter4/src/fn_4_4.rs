// 列挙型

use std::cmp::Ordering;

enum EnumExample {
    TupleTypeExample1(String),
    TupleTypeExample2(i32, bool),
    StructTypeExample{ name: String, age: u8 },
}

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

pub fn fn_4_4() {
    print_sign(determin_sign(1));

    let v= vec![
        EnumExample::TupleTypeExample1(String::from("Hello")),
        EnumExample::TupleTypeExample2(10, true),
        EnumExample::StructTypeExample {
            name: String::from("taro"),
            age: 10,
        },
    ];

    for e in &v {
        // eがStructTypeExampleの場合に表示
        if let EnumExample::StructTypeExample { name: n, age: a } = e {
            println!("StructTypeExample_iflet: name = {}, age = {}", n, a);
        }
    }

    for e in &v {
        match e {
            // eがTupleTypeExample1の場合に表示
            EnumExample::TupleTypeExample1(s) => {
                println!("TupleTypeExample1: s = {}", s);
            }
            // eがTupleTypeExample2の場合に表示
            EnumExample::TupleTypeExample2(n, b) => {
                println!("TupleTypeExample2: n = {}, b = {}", n, b);
            }
            // eがStructTypeExampleの場合に表示
            EnumExample::StructTypeExample{ name: n, .. } => {
                println!("StructTypeExample: name = {}", n);
            }
        }
    }
}