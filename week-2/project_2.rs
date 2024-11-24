fn main() {
	let toshiba:f32 = 450_000.0;
	let mac:f32 = 1_500_000.0;
	let hp:f32 = 750_000.0;
	let dell:f32 = 2_850_000.0;
	let acer:f32 = 250_000.0;
	let qty:f32 = 2.0+ 1.0+ 3.0+ 3.0+ 1.0;

	let sum:f32 = toshiba + mac + hp + dell + acer;
	println!("Sum is {}",sum);

	let average:f32 = sum / qty;
	println!("Average is {}", average);

}