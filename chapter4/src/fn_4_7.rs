static BUFFSIZE: usize = 1024;
static OFFSET: usize = 15;

pub fn fn_4_7() {
    //  メモリへの値の配置
    let boxed = Box::new(1);
    let val = *boxed;
    println!("val = {}", val);
    println!("boxed = {}", boxed);

    let recursive_enum = RecursiveEnum::Val(Box::new(RecursiveEnum::Null));
    println!("{:?}", recursive_enum);

    add_static();
    
    let offset_ref = &OFFSET;
    
    println!("buff size = {}", BUFFSIZE);
    println!("offset ref = {}", offset_ref);
}

#[derive(Debug)]
enum RecursiveEnum {
    Val(Box<RecursiveEnum>),
    Null,
}

fn add_static() {
    const INCREMENT: usize = 2;
    println!("increment = {}", INCREMENT);
}
