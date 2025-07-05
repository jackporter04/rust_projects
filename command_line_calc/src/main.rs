use std::io::{self, Write};

fn main() {
    println!("🧮 Welcome to Rust CLI Calculator!");
    println!("Type expressions like `5 + 3` or `10 / 2` (or type `exit` to quit):");

    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Ensure prompt appears

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        match eval_expression(input) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn eval_expression(expr: &str) -> Result<f64, String> {
        let tokens: Vec<&str> = expr.split_whitespace().collect();

        if tokens.len() != 3 {
            return Err("Please enter expressions in the form: number operator number (e.g., 4 * 5)".to_string());
        }
        let left = tokens[0].parse::<f64>().map_err(|_| "Invalid number")?;
        let op = tokens[1];
        let right = tokens[2].parse::<f64>().map_err(|_| "Invalid number")?;

        match op {
            "+" => Ok(left + right),
            "-" => Ok(left - right),
            "*" => Ok(left * right),
            "/" => {
                if right == 0.0 {
                    Err("Division by zero error".into())
                } else {
                    Ok(left / right)
                }
            }
            _ => Err("Unknown operator".into()),
        }
    }
}
