use std::io;
#[derive(PartialEq)]
enum Currency {
    GBP,
    USD,
    EUR,
    JPY,
}

fn get_rate(from: &Currency, to: &Currency) -> f64 {
    match (from, to) {
        (Currency::GBP, Currency::USD) => 1.25,
        (Currency::USD, Currency::GBP) => 0.8,
        (Currency::GBP, Currency::EUR) => 1.15,
        (Currency::EUR, Currency::GBP) => 0.87,
        (Currency::USD, Currency::JPY) => 110.0,
        (Currency::JPY, Currency::USD) => 0.0091,
        _ if from == to => 1.0,
        _ => {
            println!("Conversion rate not available.");
            0.0
        }
    }
}

fn convert_currency(amount: f64, from: &str, to: &str) -> Option<f64> {
    let from_currency = match from {
        "GBP" => Currency::GBP,
        "USD" => Currency::USD,
        "EUR" => Currency::EUR,
        "JPY" => Currency::JPY,
        _ => return None,
    };
    
    let to_currency = match to {
        "GBP" => Currency::GBP,
        "USD" => Currency::USD,
        "EUR" => Currency::EUR,
        "JPY" => Currency::JPY,
        _ => return None,
    };
    
    let rate = get_rate(&from_currency, &to_currency);
    if rate == 0.0 {
        return None;
    }
    
    let converted_amount = amount * rate;
    Some(converted_amount)

}


fn main() {
    println!("Currency Converter");
    println!("Enter the amount you want to convert:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let amount: f64 = input.trim().parse().expect("Please enter a valid number");
    println!("Choose the currency you want to convert from (GBP, USD, EUR, JPY):");
    let mut from_currency = String::new();
    io::stdin().read_line(&mut from_currency).expect("Failed to read line");
    let from_currency = from_currency.trim().to_uppercase();
    println!("Choose the currency you want to convert to (GBP, USD, EUR, JPY):");
    let mut to_currency = String::new();
    io::stdin().read_line(&mut to_currency).expect("Failed to read line");
    let to_currency = to_currency.trim().to_uppercase();
    let converted_amount = convert_currency(amount, &from_currency, &to_currency);
    match converted_amount {
        Some(value) => println!("Converted amount: {:.2} {}", value, to_currency),
        None => println!("Conversion failed. Please check the currency codes."),
    }
}
