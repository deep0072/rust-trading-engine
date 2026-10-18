use rust_decimal::prelude::*;
use std::collections::HashMap;

struct MatchOrder {
    ask: Order,
    bids: Order,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Order {
    size: f64,
    time_stamp: u64,
    bid: bool,
    id: u64,
}

impl Order {
    pub fn new(size: f64, time_stamp: u64, bid: bool, id: u64) -> Self {
        Self {
            size,
            time_stamp,
            bid,
            id,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Limit {
    price: Decimal,
    orders: Vec<Order>,
    total_volume: Decimal,
}

impl Limit {
    pub fn new(price: Decimal) -> Self {
        Self {
            price,
            orders: Vec::new(),
            total_volume: dec!(0.0),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.total_volume += order.size;
        self.orders.push(order);
    }
    pub fn delete_order(&mut self, mut order: Order) {
        self.total_volume -= order.size;
        self.orders.retain(|x| x != &order);
        order.id = 0;
    }
}

struct OrderBook {
    asks: Vec<Limit>,
    bids: Vec<Limit>,
    asks_limit: HashMap<Decimal, Limit>,
    bids_limit: HashMap<Decimal, Limit>,
}

impl OrderBook {
    fn new() -> Self {
        OrderBook {
            asks: Vec::new(),
            bids: Vec::new(),
            asks_limit: HashMap::new(),
            bids_limit: HashMap::new(),
        }
    }

    fn place_order(&mut self) {} // this is used to buy or sell the order when it matched
    fn add_order(&mut self, price: Decimal, order: Order) {
        let mut limit = Limit::new(price);

        if order.bid {
            limit.add_order(order);
            self.bids.push(limit.clone());
            self.bids_limit.insert(price, limit);
        } else {
            limit.add_order(order);
            self.asks.push(limit.clone());
            self.asks_limit.insert(price, limit);
        }
    } // if match not find then order will sits in orderbook
}
