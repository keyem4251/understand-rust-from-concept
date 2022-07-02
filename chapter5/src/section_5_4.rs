pub fn fn_5_4() {
    // 関連型
    // 独立な型パラメータと従属する型パラメータをトレイトに組み込む

    // 継承
    let en = EnglishPerson;
    let sp = SpanishPerson;

    say_hello_general(&en);
    say_hello_general(&sp);

    say_thankyou_general(&en);
    say_thankyou_general(&sp);

    generi_return_type();
}

// TとSが同じサイズ(i32, u32)
// Tがi32のときにSをf64にするような意図しない実装を禁止できない
// trait IAbs<T, S> {
//     fn iabs(self) -> S;
// }

// impl IAbs<i32, u32> for i32 {
//     fn iabs(self) -> u32 {
//         if self >= 0 {
//             self as u32
//         } else {
//             -self as u32
//         }
//     }
// }


trait IAbs {
    type Output;
    fn iabs(self) -> <Self as IAbs>::Output;
}

impl IAbs for i32 {
    type Output = u32;
    fn iabs(self) -> <Self as IAbs>::Output {
        if self >= 0 {
            self as <Self as IAbs>::Output
        } else {
            (-self) as <Self as IAbs>::Output
        }
    }
}

trait SayHello {
    fn say_hello(&self);
}

trait SayThankyou {
    fn say_thankyou(&self);
}

struct EnglishPerson;
struct SpanishPerson;

impl SayHello for EnglishPerson {
    fn say_hello(&self) {
        println!("Hello");
    }
}

impl SayThankyou for EnglishPerson {
    fn say_thankyou(&self) {
        println!("Thank you");
    }
}

impl SayHello for SpanishPerson {
    fn say_hello(&self) {
        println!("Hola");
    }
}

impl SayThankyou for SpanishPerson {
    fn say_thankyou(&self) {
        println!("Gracias");
    }
}

fn say_hello_general<T: SayHello>(speaker: &T) {
    speaker.say_hello();
}

fn say_thankyou_general<T: SayThankyou>(speaker: &T) {
    speaker.say_thankyou();
}

// 抽象返却型
fn generi_return_type() -> impl std::fmt::Display {
    1
}
