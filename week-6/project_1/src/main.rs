use std::io;

fn main() {
    // Display the menu
    println!("Menu:");
    println!("P = Poundo Yam/Edinkaiko Soup\n N3,200");
    println!("F = Fried Rice & Chicken \n N3,000");
    println!("A = Amala & Ewedu Soup \n N2,500");
    println!("E = Eba & Egusi Soup \n N2,000");
    println!("W = White Rice & Stew \n N2,500");

    let mut total = 0;

    loop {
        // Input the type of food
        println!("Enter the type of food (P, F, A, E, W) or 'Q' to quit:");
        let mut order = String::new();
        io::stdin().read_line(&mut order).expect("Failed to read line");
        let order = order.trim().to_uppercase();

        if order == "Q" {
            break;
        }

        // Input the quantity
        println!("Enter the quantity:");
        let mut quantity = String::new();
        io::stdin().read_line(&mut quantity).expect("Failed to read line");
        let quantity: u32 = quantity.trim().parse().expect("Please enter a valid number");

        // Calculate the price for the selected food item
        let price = match order.as_str() {
            "P" => 3200,
            "F" => 3000,
            "A" => 2500,
            "E" => 2000,
            "W" => 2500,
            _ => {
                println!("Invalid food type selected.");
                continue;
            }
        };

        total = price * quantity;
    }

    // Apply discount if total is greater than N10,000
    if total > 10000 {
        let discount = (total as f64) * 0.05;
        total -= discount as u32;
        println!("Discount applied: N{}", discount);
    }

    println!("Total charges: N{}", total);
}