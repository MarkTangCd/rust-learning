use std::fs;

fn function() {
  println!("function");
}

mod mod1 {
  pub fn function() {
    super::function();
  }

  pub mod mod2 {
    fn function() {
        println!("mod1::mod2::function");
    }
    
    pub fn call() {
        self::function();
    }
  }
}

fn main() {
  mod1::mod2::call();

  let data = fs::read("src/main.rs").unwrap();
  println!("{}", String::from_utf8(data).unwrap());

}