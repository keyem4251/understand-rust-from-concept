pub fn fn_5_1() {
    let rect = Rectangle {
        width: 1.0,
        height: 2.0,
    };
    println!("rect area = {}", area(&rect));
    println!("rect length = {}", length(&rect));
    
    let tria = RightTriangle {
        width: 1.0,
        height: 2.0,
    };
    println!("tria area = {}", area(&tria));
    println!("tria length = {}", length(&tria));

    let line = Line {
        length: 1.0,
    };
    // println!("line area = {}", area(&line));
    println!("line length = {}", length(&line));

}

trait CalcArea {
    fn calc_area(&self) -> f64;
}

trait CalcLength {
    fn calc_length(&self) -> f64;
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

impl CalcLength for Rectangle {
    fn calc_length(&self) -> f64 {
        (self.width + self.height) * 2.0
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

impl CalcLength for RightTriangle {
    fn calc_length(&self) -> f64 {
        self.width + self.height + (self.width.powi(2) + self.height.powi(2)).sqrt()
    }
}

fn area<T: CalcArea>(x: &T) -> f64 {
    x.calc_area()
}

struct Line {
    length: f64,
}

impl CalcLength for Line {
    fn calc_length(&self) -> f64 {
        self.length
    }
}

fn length<T: CalcLength>(x: &T) -> f64 {
    x.calc_length()
}
