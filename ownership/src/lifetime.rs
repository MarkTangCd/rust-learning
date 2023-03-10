#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
}

fn bigger<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1 > str2 {
        str1
    } else {
        str2
    }
}

fn main() {
    println!("{}", bigger("a", "b"));
    
    let p = Person{ name: "Jack" };
    println!("{:?}", p);
}