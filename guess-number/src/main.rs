use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    loop {
        // get the user input
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();

        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => continue,
        };

        println!("You guessed: {}", guess_number);

        // 判断大小

        if guess_number > secret_number {
            println!("Too big");
        } else if guess_number < secret_number {
            println!("Too small");
        } else {
            println!("You win!");
            break;
        }

        // 如果正确，程序退出
    }
}