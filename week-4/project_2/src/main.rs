use std::io;

fn main() {

	let mut input1 = String::new();

	println!("\nEnter your age here:");

	io::stdin()
	.read_line(&mut input1)
	.expect("Not a valid string");
	let age:i32 = input1.trim().parse().expect("Not a valid number");

	if age >= 40 {
		println!("Your incentive is N1,560,000 per month");
	}

	else if age >= 30 && age < 40 {
        println!("Your incentive is N1,480,000 per month");
	}

	else if age < 28 && age >= 20{
		println!("Your incentive is N1,300,000 per month");
    }

    else {
    	println!("Your incentive is N100,000 as you are inexperienced");
    }
}