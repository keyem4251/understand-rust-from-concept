pub fn fn_4_7() {
    //  メモリへの値の配置
    let boxed = Box::new(1);
    let val = *boxed;
    println!("val = {}", val);
    println!("boxed = {}", boxed);

    let recursive_enum = RecursiveEnum::Val(Box::new(RecursiveEnum::Null));
    print!("{:?}", recursive_enum);
}

#[derive(Debug)]
enum RecursiveEnum {
    Val(Box<RecursiveEnum>),
    Null,
}