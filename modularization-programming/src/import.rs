mod mod1;
mod mod2;

fn main() {
  println!("{}", mod1::MESSAGE);
  println!("{}", mod2::MESSAGE2);
  println!("{}", mod2::mod2_a::NUMBER);
}