# FX Price History Simulator - WIP
The following binary can be used to create dummy historical data for various FX crosses.
I created this to practice my rust and to generate constantly changing prices for an FX Trading game I am working on. 

Disclaimer - This _should not_ be used for anything that requires accuracy like backtesting an algorithm ect.
It's use is intended solely for creating games or having fun.

## Arguments
The binary requires you to pass 2 arguments.

1) A Currency Pair from the below list:
   1) `GBP/USD`
   2) `EUR/USD`
   3) `EUR/GBP`
2) The number of prices you wish to generate. E.G. `100`.

So for example, you could pass `cargo run -- GBP/USD 1200`
