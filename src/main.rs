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
		println!("Welcome to mastermind. To play enter 1. For info enter 2. To quit enter q.");
		let mut option = String::new();
		io::stdin().read_line(&mut option)
		    .expect("Failed to read line");
		    if option.trim() == "q" {
		    	exit(0)
		    }
		let option: u32 = match option.trim().parse() {
    		Ok(num) => num,
    		Err(_) => {
    			println!("Please enter a number.");
    			continue;
    		},
		};
		match option {
			1 => play_game(),
			2 => help_menu(),
			_ => continue,
		}

	}

}
//It's probably easier just to handle this with a number not a string. Too late now
fn play_game(){
	let mut secret_number: String = rand::thread_rng().gen_range(99999, 999999 +1).to_string();
	let mut counter:u32 = 1;
	if secret_number == "99999"{
		 secret_number = String::from("000000");
	}
	loop{
		println!("Enter 6 digit number. Or to quit enter q");
		println!("Your guess (#{}):",counter);
		counter += 1;
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        guess = guess.trim().to_string();
        if guess == "q"{
        	println!("Ok here is the right answer: {}", secret_number);
        	break;
        }
        if guess == secret_number{
        	println!("You Win!");
        	break;
        }
        let mut wrong_position: u32 = 0;
        let mut right_position: u32 = 0;
        if guess.parse::<u32>().is_err(){
        	println!("Has to be a number.");
        	continue;
        }
        let mut indexs_marked: Vec<usize> = Vec::new();
        for (i, c) in guess.chars().enumerate() {
            if secret_number.contains(c){
            	if secret_number.chars().nth(i).unwrap() == c {
            		right_position += 1;
            		if indexs_marked.contains(&i){
            			wrong_position -= 1;
            		} else {
            			indexs_marked.push(i);
            		}
            	} else {
            		let index =  secret_number.find(c).unwrap();
					if ! (indexs_marked.contains(&index)){
            			wrong_position +=1;
            			indexs_marked.push(index);
            		}
            	}

            	
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