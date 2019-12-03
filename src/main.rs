use std::process::exit;
use std::env;
use std::io;
use rand::Rng;
fn main() {
	for argument in env::args() {
     if argument == "bot" {
     	play_game();
     	exit(0);
     }
	}
	loop {
		println!("Welcome to mastermind. To play enter 1. For info enter 2. To quit enter 3.");
		let mut option = String::new();
		io::stdin().read_line(&mut option)
		    .expect("Failed to read line");
		let option: u32 = option.trim().parse().expect("A Number");
		match option {
			1 => play_game(),
			2 => help_menu(),
			3 => exit(0),
			_ => exit(0),
		}

	}

}
//It's probably easier just to handle this with a number not a string. Too late now
fn play_game(){
	let secret_number: String = rand::thread_rng().gen_range(100000, 999999 +1).to_string();
	let mut counter:u32 = 1;
	loop{
		println!("Enter 6 digit number. Or to quit enter anything other than a 6 digit number ex (6)");
		println!("Your guess (#{}):",counter);
		counter += 1;
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        if guess.trim() == secret_number{
        	println!("You Win!");
        	break;
        }
        let mut wrong_position: u32 = 0;
        let mut right_position: u32 = 0;
        guess = guess.trim().to_string();
        if guess.len() != 6{
        	println!("Ok here is the right answer: {}", secret_number);
        	break;
        }
        let mut secret_chars_left = secret_number.clone();
        for (i, c) in guess.chars().enumerate() {
            if secret_chars_left.contains(c){
            	if secret_chars_left.chars().nth(i).unwrap() == c {
            		right_position += 1;
            		
            	} else {
            		wrong_position +=1;
            	}
            	secret_chars_left =  secret_chars_left.replacen(c, " ",1);
            }
            
        }
        println!("{}{}", right_position,wrong_position);
        /*if i == 5 {
        	println!("You Lose!");
        }*/

	}

}
//TODO write help guide
fn help_menu()   {
	println!("You try and guess a 6 digit.");
	println!("The program return the number of digits in the right position and right but in the wrong position.");
	println!("For Example 23 would mean that 2 digits are in the correct spot and that 3 digits are correct but in the wrong spot.");
}