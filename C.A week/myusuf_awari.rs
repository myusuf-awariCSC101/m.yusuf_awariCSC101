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
   let _name = input1.trim();

   println!("\nAge: ");
   io::stdin().read_line(&mut input2).expect("Not a valid string");
   let age:i32 = input2.trim().parse().expect("Not a valid number");

   println!("\nEmail: ");
   io::stdin().read_line(&mut input3).expect("Not a valid string");
   let _email = input3.trim();
    
   println!("\nPhone Number: ");
   io::stdin().read_line(&mut input4).expect("Not a valid string");
   let _phone_number:i64 = input4.trim().parse().expect("Not a valid number");

   println!("\nNumber of siblings: ");
   io::stdin().read_line(&mut input5).expect("Not a valid string");
   let number_of_siblings:i32 = input5.trim().parse().expect("Not a valid number");

   println!("\nNumber of children: ");
   io::stdin().read_line(&mut input6).expect("Not a valid string");
   let number_of_children:i32 = input6.trim().parse().expect("Not a valid number");

   println!("\nDiagnosis: ");
   io::stdin().read_line(&mut input7).expect("Not a valid string");
   let _diagnosis = input7.trim();

   println!("\nVillage: ");
   io::stdin().read_line(&mut input8).expect("Not a valid string");
   let _village = input8.trim();

   let alzheimer = 1_200_000.0;
   let arrhythmia = 550_000.0;
   let chronic_kidney_disease = 1_500_000.0;
   let diabetes  = 800_000.0;
   let arthritis = 450_000.0;
    

if _diagnosis == "alzheimer" && age > 50 && number_of_children > 4 && _village == "akpabom"{
   let discount = 0.20;
   let subt = alzheimer * discount;
   let amount = alzheimer - subt;
   println!("This patient is to pay a total of {} ", amount);

    } 
   else if _diagnosis == "arrhythmia" && age == 30 && number_of_siblings > 4 && _village == "nigbauji"{
      let discount = 0.05; 
      let subt = arrhythmia * discount;
      let amount = arrhythmia - subt;
      println!("This patient is to pay a total of {} ", amount);
    }
   else if _diagnosis == "chronic_kidney_disease" && age > 40 && number_of_children < 3 && number_of_siblings < 3 && _village == "atabrikang"{
      let discount = 0.15;
      let subt = chronic_kidney_disease * discount;
      let amount = chronic_kidney_disease * subt;
      println!("This patient is to pay a total of {} ", amount);
    }
   else if _diagnosis == "diabetes" && age > 28 && age < 45 && number_of_children >= 2 && number_of_children <= 4 && _village == "okorobilum"{
      let discount = 0.10;
      let subt = diabetes * discount;
      let amount = diabetes * subt;
      println!("This patient is to pay a total of {} ", amount);
    } 
   else if _diagnosis == "arthritis" && age > 58 && number_of_siblings > 5 && number_of_children > 5 && _village == "emeremen"{
      let discount = 0.10; 
      let subt = arthritis * discount;
      let amount = arthritis * subt;
      println!("This patient is to pay a total of {} ", amount);
   }
   else {
      println!("No discount applied as conditions are not met.");
   }

}
