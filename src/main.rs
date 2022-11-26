extern crate core;

use rand::prelude::*;
use std::env;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        panic!("Missing command line arguments.");
    }

    let ccy_arg: &str = args[1].as_str();
    let ccy: f64 = match ccy_arg {
        "GBP/USD" => 1.21050,
        "EUR/USD" => 1.04262,
        "EUR/GBP" => 0.86670,
        _ => panic!("Invalid currency pair. Please see docs for instructions.")
    };

    println!("CCY PRICE: {}: {}",ccy_arg, ccy);

    let num_trades: i32 = (&args[2]).parse::<i32>().unwrap();
    let prices = create_next_x_prices(ccy, num_trades);

    println!("Total: {} \nData: {:?}",prices.len(), prices);

    Ok(())
}

/// Given a currency, it will simulate the next x prices.
fn create_next_x_prices(base: f64, number_to_generate: i32) -> Vec<f64> {
    let mut i: i32 = 0;
    let mut curr_price: f64 = base;
    let mut array: Vec<f64> = Vec::new();

    while i < number_to_generate {
        i += 1;

        // generates a random float between 0 - 20
        let rng: f64 = thread_rng().gen_range(0.0..20.0);
        // generates a random boolean
        let is_addition: bool = random();

        let new_price: f64 = if is_addition {
            &curr_price + (&curr_price * (rng / 100000.0))
        } else {
            &curr_price - (&curr_price * (rng / 100000.0))
        };

        // round the numbers to 5 D.P.
        let rounded_to_five_dp: f64 = (new_price * 100000f64).floor() / 100000.0;

        array.push(rounded_to_five_dp);
        curr_price = rounded_to_five_dp;
    }

    return array;
}