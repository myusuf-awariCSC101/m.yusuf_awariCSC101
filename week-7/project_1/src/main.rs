use std::io;

fn main() {
	println!("Select an equation:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");


	let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice: u32 = choice.trim().parse().unwrap();

	match choice { 

		1 => {
            println!("Enter height:");
            let mut height = String::new();
            io::stdin().read_line(&mut height).expect("Failed to read line");
	        let height:f64 = height.trim().parse().expect("Please enter a number!");
	        println!("Enter base1:");
            let mut base1 = String::new();
            io::stdin().read_line(&mut base1).expect("Failed to read line");
        	let base1:f64 = base1.trim().parse().expect("Please enter a number!");
        	println!("Enter base2:");
            let mut base2 = String::new();
            io::stdin().read_line(&mut base2).expect("Failed to read line");
	        let base2:f64 = base2.trim().parse().expect("Please enter a number!");
            let area = trapezium_area(height, base1, base2);
            println!("Area of Trapezium: {}", area);
        }

        2 => {
            println!("Enter diagonal1");
            let mut diagonal1 = String::new();
            io::stdin().read_line(&mut diagonal1).expect("Failed to read line");
	        let diagonal1:f64 = diagonal1.trim().parse().expect("Please enter a number!");
	        println!("Enter diagonal2:");
            let mut diagonal2 = String::new();
            io::stdin().read_line(&mut diagonal2).expect("Failed to read line");
        	let diagonal2:f64 = diagonal2.trim().parse().expect("Please enter a number!");
            let area = rhombus_area(diagonal1, diagonal2);
            println!("Area of Rhombus: {}", area);
        }

        3 => {
            println!("Enter base");
            let mut base = String::new();
            io::stdin().read_line(&mut base).expect("Failed to read line");
	        let base:f64 = base.trim().parse().expect("Please enter a number!");
	        println!("Enter altitude");
            let mut altitude = String::new();
            io::stdin().read_line(&mut altitude).expect("Failed to read line");
        	let altitude:f64 = altitude.trim().parse().expect("Please enter a number!");
            let area = parallelogram_area(base, altitude);
            println!("Area of Parallelogram: {}", area);
        }

        4 => {
            println!("Enter leangth of the side:");
            let mut side = String::new();
            io::stdin().read_line(&mut side).expect("Failed to read line");
	        let side:f64 = side.trim().parse().expect("Please enter a number!");
            let area = cube_area(side);
            println!("Surface Area of Cube: {}", area);
        }

        5 => {
            println!("Enter radius");
            let mut radius = String::new();
            io::stdin().read_line(&mut radius).expect("Failed to read line");
	        let radius:f64 = radius.trim().parse().expect("Please enter a number!");
	        println!("Enter height");
            let mut height = String::new();
            io::stdin().read_line(&mut height).expect("Failed to read line");
        	let height:f64 = height.trim().parse().expect("Please enter a number!");
            let volume = cylinder_volume(radius, height);
            println!("Volume of a cylinder: {}", volume);
        }

        _ => println!("Invalid choice!"),
        
    }
}

fn trapezium_area(height: f64, base1: f64, base2: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

fn rhombus_area(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

fn parallelogram_area(base: f64, altitude: f64) -> f64 {
    base * altitude
}

fn cube_area(side: f64) -> f64 {
    6.0 * side.powf(2.0)
}

fn cylinder_volume(radius: f64, height: f64) -> f64 {
    std::f64::consts::PI * radius.powf(2.0) * height
<<<<<<< HEAD
}
=======
}
>>>>>>> 0ef59a46936e2a0dbe3a7742f439091f6fdd22d0
