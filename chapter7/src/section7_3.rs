pub fn section7_3() {
    list7_1();
}

fn list7_1() {
    let v = Vec::new();
    let v = insert(2,&v);
    println!("{:?}", v);
}

fn insert(x: i32, xs: &[i32]) -> Vec<i32> {
    match xs {
        // ベクタの最初の用途をy、その他の要素（..）を@でysとして取り出す
        [y, ys @ ..] => {
            if x <= *y {
                [&[x][..], xs].concat()
            } else {
                [&[*y][..], &insert(x, ys)].concat()
            }
        }
        [] => vec![x],
    }
}
