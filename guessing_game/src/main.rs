use std::io;
use std::rand;

fn main() {
    println!("Guess the number!");
    let secret_number = (rand::random::<uint>() % 100u) + 1u;

    loop {
        print!("What is your guess: ");
        let input = io::stdin().read_line()
                               .ok()
                               .expect("Failed to read line");

        let input_num: Option<uint> = from_str(input.as_slice().trim());
        let num = match input_num {
            Some(num) => num,
            None      => {
                println!("Thats not a number!");
                continue;
            }
        };

        match cmp(num, secret_number) {
            Less    => println!("Too low!"),
            Greater => println!("Too high!"),
            Equal   => {
                println!("You win!!!");
                return;
            },
        }
    }
}

fn cmp(a: uint, b: uint) -> Ordering {
    if a < b { Less }
    else if a > b { Greater }
    else { Equal }
}
