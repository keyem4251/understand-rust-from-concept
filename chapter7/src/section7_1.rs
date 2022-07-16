pub fn section7_1() {
    list7_1();
    list7_2();
    list7_3();
    list7_4();
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

fn list7_2() {
    let vv = vec![1, 2, 3, 4];
    let mut iter = (&vv).iter();

    let x = iter.next().unwrap();
    println!("{}", x);

    let x = iter.next().unwrap();
    println!("{}", x);

    // 参照のためエラーにならない
    let x = vv[2];
    println!("{}", x);
}

fn list7_3() {
    let mut vv = vec![1, 2, 3, 4];
    let mut iter = (&mut vv).iter_mut();

    let x = iter.next().unwrap();
    println!("{}", x);

    let x = iter.next().unwrap();
    println!("{}", x);

    *x += 10;
    println!("{:?}", vv);
}

fn list7_4() {
    let vv = vec![1, 2, 3, 4];

    for ii in &vv {
        println!("{}", ii);
    }

    for ii in &vv {
        println!("{}", ii);
    }
}
