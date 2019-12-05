use rand::Rng;
use std::env;
use std::io;
use std::process::exit;
fn main() {
    for argument in env::args() {
        if argument == "bot" {
            play_game();
            exit(0);
        }
    }
    loop {
        println!("Welcome to mastermind. To play enter 1. For info enter 2. To quit enter q.");
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");
        if option.trim() == "q" {
            exit(0)
        }
        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number.");
                continue;
            }
        };
        match option {
            1 => play_game(),
            2 => help_menu(),
            _ => continue,
        }
    }
}
fn play_game() {
    let mut rand_generator = rand::thread_rng();
        let secret_number: [u32; 6] = [
        rand_generator.gen_range(0, 10),
        rand_generator.gen_range(0, 10),
        rand_generator.gen_range(0, 10),
        rand_generator.gen_range(0, 10),
        rand_generator.gen_range(0, 10),
        rand_generator.gen_range(0, 10),
    ];
    let secret_number_string: String = secret_number.iter().fold(String::new(), |mut result, x| {
        result.push_str(&x.to_string());
        result
    });
    let mut counter: u32 = 1;
    loop {
        println!("Enter 6 digit number. Or to quit enter q");
        println!("Your guess (#{}):", counter);
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        //variable shadow avoid copying  creates slice of orignal
        let guess = guess.trim();
        if guess == "q" {
            println!("Ok here is the right answer: {}", secret_number_string);
            break;
        }
        if (!guess.chars().all(|x| char::is_ascii_digit(&x))) || guess.len() != 6 {
            println!("Has to be a  6 digit number.");
            continue;
        }
        let mut guess_digits: Vec<u32> = guess.chars().map(|x| x.to_digit(10).unwrap()).collect();
        if secret_number.iter().eq(guess_digits.iter()) {
            println!("{}", secret_number_string);
        }
        let mut wrong_position: u32 = 0;
        let mut right_position: u32 = 0;
        let mut secret_number_clone = secret_number;
        for (i, val) in secret_number.iter().enumerate() {
            if guess_digits[i] == *val {
                right_position += 1;
                secret_number_clone[i] = 11;
                guess_digits[i] = 12;
            }
            
        }
        for val in guess_digits.iter() {
            if  secret_number_clone.contains(val){
                let index: usize =  secret_number_clone.iter().position(|x| x == val).unwrap_or(7);
                if index != 7{
                	wrong_position += 1;
	                secret_number_clone[index] = 11;
            	}
            }
        }
        counter += 1;
        println!("{}{}", right_position, wrong_position);
        /*if i == 5 {
            println!("You Lose!");
        }*/
    }
}
//TODO write help guide
fn help_menu() {
    println!("You try and guess a 6 digit.");
    println!("The program return the number of digits in the right position and right but in the wrong position.");
    println!("For Example 23 would mean that 2 digits are in the correct spot and that 3 digits are correct but in the wrong spot.");
}
