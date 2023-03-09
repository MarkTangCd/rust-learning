#[derive(Debug, PartialEq, Default)]
struct Point {
  x: i32,
  y: i32,
}

fn main() {
  let p = Point { x: 32, y: 12 };
  let p2 = Point { x: 32, y: 22 };
  println!("{:?}", p);
  println!("{}", p == p2);
  
  let p3 = Point::default();
  println!("{:?}", p3);
}