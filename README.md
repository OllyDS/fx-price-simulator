# FX Price History Simulator - WIP
The following crate can be used to create dummy historical data for various FX crosses.
This project was created so that I could improve my rust and to generate random prices for an FX Trading game I am working on. 

Disclaimer - This _should not_ be used for any real-world applications such as backtesting the performance of an algorithm etc.
It's use is intended solely for creating games, demos or having fun.

## Available functions

### generate_fx_prices()
The binary requires you to pass 2 arguments.

2) A Currency Pair from the below list:
   1) `GBP/USD`
   2) `EUR/USD`
   3) `EUR/GBP`

2) The number of prices you wish to generate. E.G. `100`.

You can also test this from the commmand line by running `cargo run -- GBP/USD 120`
