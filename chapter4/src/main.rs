fn main() {
    fn_4_1();
    fn_4_2();
}

fn fn_4_1() {
    // &strからStringに変換して文字列を操作する
    let mut st1 = "Hello".to_string();
    st1.push_str(", World");
    println!("{}", st1);
    // sliceで取り出すにはリファレンスにする
    println!("{}", &st1[0..6]);
}

fn fn_4_2() {
    struct Person {
        name: String,
        age: u8,
    }

    let taro = Person{
        name: "taro".to_string(),
        age: 10,
    };

    let jiro = Person {
        name: "jiro".to_string(),
        ..taro
    };
    println!("name: {}, age: {}", jiro.name, jiro.age);

    // タプル構造体
    struct Celsius(f64); // 摂氏温度
    struct Kelvin(f64); // 絶対温度

    let temp_celsius = Celsius(10.0);
    let temp_kelvin = Kelvin(10.0);

    println!("celsius: {}, kelvin: {}", temp_celsius.0, temp_kelvin.0);
}
