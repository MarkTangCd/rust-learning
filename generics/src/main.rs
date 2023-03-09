struct Point<T> {
    x: T,
    y: T,
}

impl<T: std::cmp::PartialOrd + Clone> Point<T> {
    fn largest(&self) -> T {
        if self.x > self.y {
            self.x.clone()
        } else {
            self.y.clone()
        }
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let point = Point {x: 10, y: 20};
    println!("{:?}", point.largest());
    
    let point2 = Point {x: 10.0, y: 20.0};
    println!("{:?}", point2.distance_from_origin());
}