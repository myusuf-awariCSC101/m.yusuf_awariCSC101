use std::io;

fn main() {
	let mut input1 = String::new()

	println!("Welcome to the resturant!")

	println!("MENU");
	println!("P = Poundo Yam & Edinkaiko Soup \nN3,200");
	println!("F = Fried Rice & Chicken \nN3,000");
	println!("A = Amala & Ewedu Soup \nN2,500");
	println!("E = Eba & Egusi Soup \nN2,000");
	println!("W = White Rice & Stew \nN2,500");

	let P = 3200;
	let F = 3000;
	let A = 2500;
	let E = 2000;
	let W = 2500;

	println!("Enter Your Order");
	io::stdin()
	.read_line(&mut input1)
	.expect("Not a valid string");

	if total_cost > 10000 {
		let discount = total_cost * 0.05;
		let total_cost = total_cost - discount;

	}
	println!("Congratulation, you have received a 5% discount");
	println!("Your bill is {}, total_cost ");
	println!("Thank you!");
}