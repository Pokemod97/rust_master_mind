use mastermind_core;
use std::collections::HashMap;
use std::env;
use std::io;
use std::process::exit;
fn main() {
    for argument in env::args() {
        if argument == "bot".to_string() {
            play_game();
            exit(0);
        }
    }
    loop {
        println!("Welcome to mastermind. To guess enter 1.  For info enter 2. To be the secret keeper press 3. To quit enter q.");
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
            3 => play_human(),
            _ => continue,
        }
    }
}
fn print_2_digit() {
    println!("Please enter corrsponding 2 digits.");
}
fn play_human() {
    let mut guess: [u32; 6];
    let mut win: bool = false;
    let mut zero = 11;
    let mut numbers: HashMap<u32, u32> = HashMap::new();
    let mut times = 0;
    for i in 0..10 {
        guess = [i; 6];
        print_2_digit();
        println!("{}", mastermind_core::vec_to_string(guess.to_vec()));
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line");
        if response.trim() == "q" {
            return;
        }
        if (!response.trim().chars().all(|x| char::is_ascii_digit(&x)))
            || response.trim().len() != 2
        {
            println!("Incorrect option please start over.");
            continue;
        }
        if response.trim() == "60" {
            win = true;
            break;
        } else if response.trim() == "00" {
            zero = i;
        } else {
            numbers.insert(
                i,
                response
                    .trim()
                    .chars()
                    .nth(0)
                    .unwrap()
                    .to_digit(10)
                    .unwrap(),
            );
            times += response
                .trim()
                .chars()
                .nth(0)
                .unwrap()
                .to_digit(10)
                .unwrap();
        }
        if times == 6 && zero != 11 {
            break;
        }
    }
    let mut result: [u32; 6] = [0; 6];
    if !win {
        for (key, value) in numbers {
            if win {
                break;
            }
            let mut times_found = 0;
            for i in 0..6 {
                let mut new_guess: [u32; 6] = [zero; 6];
                new_guess[i as usize] = key;
                print_2_digit();
                println!("{}", mastermind_core::vec_to_string(new_guess.to_vec()));
                let mut response = String::new();
                io::stdin()
                    .read_line(&mut response)
                    .expect("Failed to read line");
                if response.trim() == "q" {
                    return;
                }
                if (!response.trim().chars().all(|x| char::is_ascii_digit(&x)))
                    || response.trim().len() != 2
                {
                    println!("Incorrect option please start over.");
                    return;
                }
                if response.trim() == "10" {
                    result[i as usize] = key;
                    times_found += 1;
                }
                if response.trim() == "60" {
                    win = true;
                    break;
                }
                if times_found == value {
                    break;
                }
            }
        }
    }
    print_2_digit();
    println!("{}", mastermind_core::vec_to_string(result.to_vec()));
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    if response.trim() == "q" {
        return;
    }
    if response.trim() == "60" {
        win = true;
    }
    if win {
        println!("Yay I guessed it!");
    }
}
fn play_game() {
    let secret_number = mastermind_core::generate_secret_number();
    let secret_number_string: String = mastermind_core::vec_to_string(secret_number.to_vec());
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
        let guess_digits: Vec<u32> = guess.chars().map(|x| x.to_digit(10).unwrap()).collect();
        if secret_number.iter().eq(guess_digits.iter()) {
            println!("{}", secret_number_string);
        }

        counter += 1;
        let positions = mastermind_core::positions(secret_number.to_vec(), guess_digits);
        println!("{}{}", positions.0, positions.1);
        /*if i == 5 {
            println!("You Lose!");
        }*/
    }
}
fn help_menu() {
    println!("You try and guess a 6 digit.");
    println!("The program return the number of digits in the right position and right but in the wrong position.");
    println!("For Example 23 would mean that 2 digits are in the correct spot and that 3 digits are correct but in the wrong spot.");
}
