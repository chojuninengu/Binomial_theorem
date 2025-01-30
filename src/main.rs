use std::io;

// Function to calculate the binomial coefficient, aka "n choose k"
fn binomial_coefficient(n: u64, k: u64) -> f64 
{

        if k > n 
        {
            return 0.0;
        }


        // to minimize the number of multiplications
        let k = std::cmp::min(k, n - k);


        let mut result = 1.0;

        for i in 0..k 
        {
            result *= (n - i) as f64; // Multiply by (n - i)
            result /= (i + 1) as f64; // Divide by (i + 1)
        }

        result

}

// Function for the binomial expansion of (a + b)^n
fn binomial_theorem(a: f64, b: f64, n: u64) -> Vec<f64> 
{

    // Generate the coefficients for each term in the expansion
    (0..=n)
        .map(|k| {
            let coeff = binomial_coefficient(n, k); 
            coeff * a.powi((n - k) as i32) * b.powi(k as i32) // Calculate the term
        })
        
        .collect() // Collect all terms into a vector

}

// Main function where the magic happens
fn main() {
    println!("Welcome to the Binomial Theorem Calculator!");
    println!("This program calculates the coefficients of (a + b)^n.");

    // Get user input for a, b, and n
    let a = prompt_for_number("Enter the value of a ");
    let b = prompt_for_number("Enter the value of b ");
    let n = prompt_for_number("Enter the value of n ") as u64;

    // Compute the binomial expansion
    let expansion = binomial_theorem(a, b, n);

    // Display the result
    println!("\nHere are the coefficients of ({} + {})^{}:", a, b, n);
    println!("{:?}", expansion);

    println!("\nThanks for using the Binomial Theorem Calculator! Have a great day!");
}

// Helper function to prompt the user for a number and parse it
fn prompt_for_number(prompt: &str) -> f64 {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Oops! Failed to read your input.");

    // Try to parse the input as a number
    input
        .trim()
        .parse()
        .unwrap_or_else(|_| {
            println!("That doesn't look like a valid number. Let's try again!");
            prompt_for_number(prompt) // Recursively prompt again if input is invalid
        })
}