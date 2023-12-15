use std::io;

pub fn ask_num_players() -> i32 {
    loop {
        println!("How many players are going to play?");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) if num >= 1 && num <= 2 => return num,
            Ok(_) => println!("Please enter a number between 1 and 2"),
            Err(_) => println!("Please enter a valid number"),
        }
    }
}
