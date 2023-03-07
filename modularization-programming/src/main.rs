mod mod1 {
    pub struct Person {
        pub name: String,
        nickname: String,
    }

    impl Person {
        pub fn new(name: &str) -> Self {
            Person { name: String::from(name), nickname: String::new() }
        }

        pub fn set_nickname(&mut self, nickname: &str) {
            self.nickname = String::from(nickname);
        }

        pub fn get_nickname(&self) {
            println!("{}", self.nickname);
        }
    }

    pub const MESSAGE: &str = "Hello World!";
    pub(self) const NUMBER: u32 = 42;

    pub(crate) enum CrateEnum {
        Item = 4
    }

    pub mod mod2 {
        pub const MESSAGE: &str = "Hello World!";

        pub fn say42() {
            println!("{}", super::NUMBER);
        }
    }
}

fn main() {
    let mut p = mod1::Person::new("Mark");
    p.set_nickname("mm");
    p.get_nickname();

    println!("{}", mod1::mod2::MESSAGE);
    println!("{}", mod1::CrateEnum::Item as u32);
    mod1::mod2::say42();

    // mod1::NUMBER, Error
}
