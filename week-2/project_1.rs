fn main() {
	let p:f32 = 520_000_000.0;
	let t:f32 = 5.0;
	let r:f32 =10.0;

	let a:f32 = p * (1.0 + (r / 100.0)) * t;
	let ci:f32 = a - p;
	println!("Compound interst is {}", ci)
}