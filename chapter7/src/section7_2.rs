pub fn section7_2() {
    println!("{}", fact(3));

    let vv = vec![0, 1, 2, 3];
    println!("{:?}", list7_1(vv));

    list7_2();
    list7_3();
}

fn fact(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * fact(n - 1)
    }
}

fn list7_1(vv: Vec<i32>) -> Vec<i32> {
    let mut ret = Vec::new();

    for ee in vv {
        ret.push(ee + 1);
    }

    ret 
}

fn add_one(x :i32) -> i32 {
    x + 1
}

fn list7_2() {
    let v: Vec<i32> = vec![0, 1, 2, 3].into_iter().map(add_one).collect();
    println!("{:?}", v);
}

fn list7_3() {
    let v: Vec<i32> = vec![0, 1, 2, 3].into_iter().map(|x| x + 1).collect();
    println!("{:?}", v);
}
