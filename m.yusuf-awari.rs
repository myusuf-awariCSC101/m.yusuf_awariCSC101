use std::io;

fn main() {
	let mut input1= String::new();
    1et mut input2= String::new();
    let mut input3= String::new();
    let mut input4= String::new();
    let mut input5= String::new();
    let mut input6= String::new();
    let mut input7= String::new();
    let mut input8= String::new();


    println!("Name: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Age: ");
    io::stdin(.read_line(&mut input2).expect("Not a valid string");
    let Age:i32 = input2.trim().parse().expect("Not a valid number");

    println!("Email: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    
    println!("Phone Number: ");
    io::stdin().read_line(%mut input4).expect("Not a valid string");
    let Phone Number:i32 = input4.trim().parse().expect("Not a valid number");

    println!("Number of siblings: ");
    io::stdin().read_line(%mut input5).expect("Not a valid string");
    let Number of siblings:i32 = input5.trim().parse().expect("Not a valid number");

    println!("Number of children: ");
    io::stdin().read_line(%mut input6).expect("Not a valid string");
    let Number of children:i32 = input6.trim().parse().expect("Not a valid number");

    println!("Diagnosis: ");
    io::stdin().read_line(%mut input7).expect("Not a valid string");

    println!("Village: ");
    io::stdin().read_line(%mut input8).expect("Not a valid string");

    let Diagnosis: {
        "Alzheimer" => 1_200_000.0,
        "Arrhythmia" => 750_000.0,
        "Chronic Kidney Disease" => 1_500_000.0,
        "Diabetes" => 500_000.0,
        "Arthritis" => 350_000.0,
        _ => {
            println!("Unknown diagnosis. Please check input.");
            return;
    }

if diagnosis = "Alzheimer" 
   age > 50 
   number_of_children > 4 
   village = "Ajakotom" {
   discount = 0.20; 
    } else 
if diagnosis = "Arrhythmia"
   age = 30 
   number_of_siblings > 4 
   village = "Nigeriku" {
   discount = 0.05; 
    }else
if diagnosis = "Chronic Kidney Disease"
   age > 40 
   number_of_children < 3
   number_of_siblings < 3 
   village = "Abarikang" {
   discount = 0.15; 
    }else
if diagnosis = "Diabetes" 
    age > 28
    age < 45 
    number_of_children >= 2 
    number_of_children <= 4 
    village = "Okorobiko" {
    discount = 0.10; 
    } else 
if diagnosis = "Arthritis"
   age > 58 
   number_of_siblings > 5 
   village = "Emematan" {
   discount = 0.10; 
}
 
 let final_amount = amount * (1.0 - discount);

    println!(" Patient Information: ");
    println!("Name: {}", input1.String();
    println!("Age: {}", age);
    println!("Email: {}", input3.String());
    println!("Phone Number: {}", phone_number);
    println!("Number of siblings: {}", number_of_siblings);
    println!("Number of children: {}", number_of_children);
    println!("Medical Diagnosis: {}", diagnosis);
    println!("Village: {}", village);
    println!("Discount applied: {:.0}%", discount * 100.0);
    println!("Total amount to pay: {:.2} Naira", final_amount)

}