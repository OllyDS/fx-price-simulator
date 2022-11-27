use ::fx_trade_gen::{create_next_x_prices,determine_price_move,generate_fx_prices};

#[test]
fn test_determine_price_move() {
    let price_move: f64 = determine_price_move();

    assert!(
        price_move >= 0.0,
        "price_move was {}, but should be greater than/equal to 0.0",
        price_move
    );
    assert!(
        price_move <= 50.0,
        "price_move was {}, but should be less than/equal to 50.0",
        price_move
    );
}

#[test]
fn test_create_next_fifty_prices() {
    let gbp_usd_prices: Vec<f64> = create_next_x_prices(1.25000, 50);

    assert_eq!(gbp_usd_prices.len(), 50);
    assert!(gbp_usd_prices[0] > 1.24 && gbp_usd_prices[0] < 1.26);
}

#[test]
fn test_create_next_zero_prices() {
    let eur_usd_prices: Vec<f64> = create_next_x_prices(1.0525, 0);

    assert_eq!(eur_usd_prices.len(), 0);
}

#[test]
fn test_generate_fx_prices() {
    let prices = generate_fx_prices("EUR/GBP", 10);

    assert_eq!(prices.len(), 10);
}

#[test]
fn test_generate_fx_prices_zero_iterations() {
    let prices = generate_fx_prices("EUR/GBP", 0);

    assert_eq!(prices.len(), 0);
}

#[test]
#[should_panic(expected = "Invalid currency pair. Please see docs for instructions.")]
fn test_generate_fx_prices_err() {
    generate_fx_prices("USD/ZAR", 10);
}
