use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::ops::Add;

pub fn fn_5_3() {
    let x = Point2d {
        x: 3.0,
        y: 2.0,
    };
    let y = Point2d {
        x: 6.0,
        y: 3.0,
    };
    let z = Point2d {
        x: 9.0,
        y: 8.0,
    };

    println!("x + y = {:?}", x + y);
    println!("x > y ? : {}", x > y);
    println!("x == z ? : {}", x == z);
}

#[derive(Debug, Clone, Copy)]
struct Point2d {
    x: f64,
    y: f64,
}

impl Point2d {
    fn distance_sql(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }
}

impl Add for Point2d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl PartialEq for Point2d {
    fn eq(&self, other: &Self) -> bool {
        let dist_self_sq = self.distance_sql();
        let dist_other_sq = other.distance_sql();
        dist_self_sq.eq(&dist_other_sq)
    }
}

impl PartialOrd for Point2d {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let dist_self_sq = self.distance_sql();
        let dist_other_sq = other.distance_sql();
        dist_self_sq.partial_cmp(&dist_other_sq)
    }
}
