fn main() {
	let a:i32 = 10;
	let b:i32 = 20;

	println!("Value of a {}", a);
	println!("Value of b: {}", b);

	let mut res = a > b;
	println!("a greater than b: {}", res);

	res = a < b;
	println!("a lesser than b: {}", res);

	res = a >= b;
	println!("a greater than or equals to b: {}", res);

	res = a <= b;
	println!("a less than or equals to b: {}", res);

	res = a == b;
	println!("a is equals to b: {}", res);

	res = a != b;
	println!("a is not equals to b: {}", res);
}