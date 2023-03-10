fn echo(a: &str) {
    println!("{}", a);
}

fn main() {
    let s = String::from("Hello World");
    echo(&s);

    println!("{}", s);
}
