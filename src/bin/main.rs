use std::env;

use ::fx_trade_gen::generate_fx_prices;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        panic!("Missing command line arguments.");
    }

    let ccy: &str = args[1].as_str();
    let num_trades: i32 = (&args[2]).parse::<i32>().unwrap();

    let fx_prices: Vec<f64> = generate_fx_prices(ccy, num_trades).unwrap();
    println!("Total: {} \nData: {:?}",fx_prices.len(), fx_prices);
}
