fn main() {

	let name = "Aisha Lawal";
	let uni:&str = "Pan-Atlantic University";
	let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";

	println!("Name: {}", name);
	println!("University: {}, \nAddress: {}", uni, addr);


	let department:&'static str = "Computer Science";
	let school:&'static str = "School of Science and technology";
	println!("Department: {}, \nSchool: {}", department, school);

}
