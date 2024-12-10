use std::io;

fn main() {
	let mut input1= String::new();
   let mut input2= String::new();
   let mut input3= String::new();
   let mut input4= String::new();
   let mut input5= String::new();
   let mut input6= String::new();
   let mut input7= String::new();
   let mut input8= String::new();


   println!("\nName: ");
   io::stdin().read_line(&mut input1).expect("Not a valid string");

   println!("\nAge: ");
   io::stdin(.read_line(&mut input2).expect("Not a valid string");
   let Age:i32 = input2.trim().parse().expect("Not a valid number");

   println!("\nEmail: ");
   io::stdin().read_line(&mut input3).expect("Not a valid string");
    
   println!("\nPhone Number: ");
   io::stdin().read_line(&mut input4).expect("Not a valid string");
   let Phone Number:i32 = input4.trim().parse().expect("Not a valid number");

   println!("\nNumber of siblings: ");
   io::stdin().read_line(&mut input5).expect("Not a valid string");
   let Number of siblings:i32 = input5.trim().parse().expect("Not a valid number");

   println!("\nNumber of children: ");
   io::stdin().read_line(&mut input6).expect("Not a valid string");
   let Number of children:i32 = input6.trim().parse().expect("Not a valid number");

   println!("\nDiagnosis: ");
   io::stdin().read_line(&mut input7).expect("Not a valid string");

   println!("\nVillage: ");
   io::stdin().read_line(&mut input8).expect("Not a valid string");

   let Alzheimer = 1_200_000.0;
   let Arrhythmia = 550_000.0;
   let Chronic Kidney Disease = 1_500_000.0;
   let Diabetes  = 800_000.0;
   let Arthritis = 450_000.0;
    

if diagnosis = "Alzheimer" && age > 50 && number_of_children > 4 && village = "Akpabom"; {
   let discount = 0.20;
   let subt = Alzheimer * discount;
   let amount = Alzheimer - subt;
   println!("This patient is to pay a total of {} ", amount);

    } 
   else if diagnosis = "Arrhythmia" && age = 30 && number_of_siblings > 4 && village = "Nigbauji"; {
      let discount = 0.05; 
      let subt = Arrhythmia * discount;
      let amount = Arrhythmia - subt;
      println!("This patient is to pay a total of {} ", amount);
    }
   else if diagnosis = "Chronic Kidney Disease" && age > 40 && number_of_children < 3 && number_of_siblings < 3 && village = "Atabrikang" {
      let discount = 0.15;
      let subt = Chronic Kidney Disease * discount;
      let amount = Chronic Kidney Disease * subt;
      println!("This patient is to pay a total of {} ", amount);
    }
   else if diagnosis = "Diabetes" && age > 28 && age < 45 && number_of_children >= 2 && number_of_children <= 4 && village = "Okorobilum" {
      let discount = 0.10;
      let subt = Diabetes * discount;
      let amount = Diabetes * subt;
      println!("This patient is to pay a total of {} ", amount);
    } 
   else if diagnosis = "Arthritis" && age > 58 && number_of_siblings > 5 && number_of_children > 5 && village = "Emeremen" {
      let discount = 0.10; 
      let subt = Arthritis * discount;
      let amount = Arthritis * subt;
      println!("This patient is to pay a total of {} ", amount);
}

}