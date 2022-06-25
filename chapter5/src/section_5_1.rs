pub fn fn_5_1() {
    let rect = Rectangle {
        width: 1.0,
        height: 2.0,
    };
    println!("rect area = {}", area(&rect));

    let tria = RightTriangle {
        width: 1.0,
        height: 2.0,
    };
    println!("tria area = {}", area(&tria));
}

trait CalcArea {
    fn calc_area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl CalcArea for Rectangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height
    }
}

struct RightTriangle {
    width: f64,
    height: f64,
}

impl CalcArea for RightTriangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height * 0.5
    }
}

fn area<T: CalcArea>(x: &T) -> f64 {
    x.calc_area()
}
