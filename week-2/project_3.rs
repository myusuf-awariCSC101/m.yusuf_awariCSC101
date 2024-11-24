fn main() {
    let p: f64 = 510_000.0; // Initial value of the TV in Naira
    let r: f64 = 5.0;   // Depreciation rate per annum in percentage
    let n: f64 = 3.0;                 // Number of years

    // Calculate the depreciated value using the formula
    let depreciated_value = p * (1.0 - (r / 100.0)).powi(n as i32);

    // Print the result
    println!("The value of the TV after 3 years is ₦{}", depreciated_value);
}