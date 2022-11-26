extern crate core;

use rand::prelude::*;
use std::env;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        panic!("Missing command line arguments.");
    }

    let ccy: &str = args[1].as_str();
    let num_trades: i32 = (&args[2]).parse::<i32>().unwrap();
    let spot_price: f64 = match ccy {
        "GBP/USD" => 1.21050,
        "EUR/USD" => 1.04262,
        "EUR/GBP" => 0.86670,
        _ => panic!("Invalid currency pair. Please see docs for instructions.")
    };
    println!("CCY PRICE: {}: {}",ccy, spot_price);

    let prices = create_next_x_prices(spot_price, num_trades);
    println!("Total: {} \nData: {:?}",prices.len(), prices);

    Ok(())
}

/// Given a currency, it will simulate x historical prices.
fn create_next_x_prices(base: f64, number_to_generate: i32) -> Vec<f64> {
    let mut idx: i32 = 0;
    let mut curr_price: f64 = base;
    let mut prices: Vec<f64> = Vec::new();

    while idx < number_to_generate {
        idx += 1;

        let price_movement: f64 = determine_price_move();
        let new_price: f64 = determine_price_direction(&curr_price, price_movement);

        // round the numbers to 5 D.P.
        let rounded_to_five_dp: f64 = (new_price * 100000f64).floor() / 100000.0;

        prices.push(rounded_to_five_dp);
        curr_price = rounded_to_five_dp;
    }

    return prices;
}

/// Determines the price movement.
/// Calculates price move and adds random chance of x standard deviation move.
fn determine_price_move() -> f64 {
    // firstly let's determine the sigma of the move.
    // This is a good primer on sigma from MIT: https://tinyurl.com/8ecf5ud5
    let sigma_range: f64 = thread_rng().gen_range(0.0..100.0);
    let sigma: i32 =
        if sigma_range < 68.2 { 1 }
        else if sigma_range > 68.2 && sigma_range < 95.8 { 2 }
        else if sigma_range > 95.8 && sigma_range < 0.2 { 3 }
        else if sigma_range > 0.2 && sigma_range < 0.000001 { 4 }
        else { 5 };

    let rng: f64 = thread_rng().gen_range(0.0..10.0);

    return rng * sigma as f64;
}

/// Given a price and price movement,
/// it will randomly determine whether the price movement is positive or negative.
fn determine_price_direction(curr_price: &f64, price_movement: f64) -> f64 {
    let is_addition: bool = random();

    return
        if is_addition { curr_price + (*&curr_price * (price_movement / 100000.0)) }
        else { curr_price - (*&curr_price * (price_movement / 100000.0)) };
}