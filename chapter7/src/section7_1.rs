pub fn section7_1() {
    list7_1();
}

fn list7_1() {
    let vv = vec![1, 2, 3, 4];
    let mut iter = vv.into_iter();

    let x = iter.next().unwrap();
    println!("{}", x);

    let x = iter.next().unwrap();
    println!("{}", x);

    // 所有権が移動しているためエラー
    // let x = vv[2];
    // println!("{}", x);
}