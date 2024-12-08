use std::io;

fn main() {

	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();

    //prompt for coefficients
	println!("Enter the coefficient a: ");
	io::stdin()
	.read_line(&mut input1)
	.expect("Failed to read input");

	println!("Enter the coefficient b: ");
	io::stdin()
	.read_line(&mut input2)
	.expect("Failed to read input");

	println!("Enter the coefficient c: ");
	io::stdin()
	.read_line(&mut input3)
	.expect("Failed to read input");

    //Convert inputs to numbers
	let a:f64 = input1
	.trim()
	.parse()
	.expect("Please enter a valid number");

	let b:f64 = input2
	.trim()
	.parse()
	.expect("Please enter a valid number");

	let c:f64 = input3
	.trim()
	.parse()
	.expect("please enter a valid number");

	//checking if it's a quadratic equation
	if a == 0.0 {
		println!{"This is not a quadratic equation (a cannot be zero)."}
	}

	//the discriminant
	let discriminant = b * b - 4.0 * a * c;

	if discriminant > 0.0 {
		let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        // These are the roots
        println!("Two distinct real roots: x1 = {}, x2 = {}", root1,root2);

	} else if discriminant == 0.0 {
		let root = -b / (2.0 * a);
		println!("One real root: x = {}", root);

	} else {
		println!("No real roots (discriminant is negative).");
	}


}
