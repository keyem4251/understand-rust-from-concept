fn main() {
    fn_4_1();
}

fn fn_4_1() {
    // &strからStringに変換して文字列を操作する
    let mut st1 = "Hello".to_string();
    st1.push_str(", World");
    println!("{}", st1);
    // sliceで取り出すにはリファレンスにする
    println!("{}", &st1[0..6]);
}
