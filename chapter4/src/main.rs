use chapter4::fn_4_4::fn_4_4;
use chapter4::fn_4_5::fn_4_5;
use chapter4::fn_4_6::fn_4_6;

fn main() {
    fn_4_1();
    fn_4_2();
    fn_4_3();
    fn_4_4();
    fn_4_5();
    fn_4_6();
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

fn fn_4_3() {
    #[derive(Debug)]
    struct Person {
        _name: String,
        _age: u8,
    }

    // 構造体のフィールドにリファレンスを使う
    #[derive(Debug)]
    struct Parents<'a, 'b> {
        _father: &'a Person,
        _mother: &'b Person,
    }

    // new の引数のfatherとParents.fatherが同じライフタイムを持つ
    // e.q. taroとjiro.fatherは同じライフタイムを持つ
    impl<'a, 'b> Parents<'a, 'b>{
        fn new(father: &'a Person, mother: &'b Person) -> Parents<'a, 'b> {
            Parents {
                _father: father,
                _mother: mother,
            }
        }
    }

    let taro = Person { _name: String::from("taro"), _age: 50 };
    let hanako = Person { _name: String::from("hanako"), _age: 48 };

    let jiro = Parents::new(&taro, &hanako);
    println!("{:?}", jiro);
}
