use super::orderbook::{Order, OrderBook, Price};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(quote: String, base: String) -> Self {
        TradingPair { base, quote }
    }

    pub fn to_string(self) -> String {
        return format!("{}_{}", self.base, self.quote);
    }
}

#[derive(Debug)]
pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, OrderBook>,
}

impl MatchingEngine {
    pub fn new() -> Self {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_new_market(&mut self, pair: TradingPair) {
        self.orderbooks.insert(pair.clone(), OrderBook::new());
        println!("opening orderbook for market {:?}", pair);
    }

    pub fn place_new_limit_order(
        &mut self,
        pair: TradingPair,
        price: f64,
        order: Order,
    ) -> Result<(), String> {
        match self.orderbooks.get_mut(&pair) {
            Some(orderbook) => {
                orderbook.add_order(price, order);
                Ok(())
            }
            None => {
                println!(" there is no order for orderbook");
                Err(format!(
                    "the orderbook for the given trading pair ({}) does not exist",
                    pair.to_string()
                ))
            }
        }
    }
}
