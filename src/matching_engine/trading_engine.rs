use super::orderbook::{Order, OrderBook};
use rust_decimal::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
    Base: String,
    Quote: String,
}

impl TradingPair {
    pub fn new(Base: String, Quote: String) -> Self {
        TradingPair { Base, Quote }
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

    pub fn add_new_market(&mut self, trading_pair: TradingPair) {
        self.orderbooks.insert(trading_pair, OrderBook::new());
    }

    pub fn add_limit_order(
        &mut self,
        order: Order,
        price: Decimal,
        trading_pair: TradingPair,
    ) -> Result<(), String> {
        match self.orderbooks.get_mut(&trading_pair) {
            Some(orderbook) => {
                orderbook.add_order(price, order);
                Ok(())
            }

            None => Err(format!("the orderbook is emppty for given pair")),
        }
    }
}
