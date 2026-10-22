use rust_decimal::prelude::*;
use std::collections::HashMap;

struct MatchOrder {
    ask: Order,
    bids: Order,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Order {
    size: Decimal,
    time_stamp: u64,
    bid: bool,
    id: u64,
}

impl Order {
    pub fn new(size: Decimal, time_stamp: u64, bid: bool, id: u64) -> Self {
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

#[derive(Debug)]
pub struct OrderBook {
    pub asks: Vec<Limit>,
    pub bids: Vec<Limit>,
    asks_limit: HashMap<Decimal, Limit>,
    bids_limit: HashMap<Decimal, Limit>,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            asks: Vec::new(),
            bids: Vec::new(),
            asks_limit: HashMap::new(),
            bids_limit: HashMap::new(),
        }
    }

    pub fn place_order(&mut self, price: Decimal, order: Order) {
        if order.size > dec!(0.0) {
            self.add_order(price, order);
        }
    } // this is used to buy or sell the order when it matched
    pub fn add_order(&mut self, price: Decimal, order: Order) {
        let mut limit = if order.bid {
            match self.bids_limit.get(&price) {
                Some(limit) => limit.clone(),
                None => Limit::new(price),
            }
        } else {
            match self.asks_limit.get(&price) {
                Some(limit) => limit.clone(),
                None => Limit::new(price),
            }
        };
        //
        if limit.orders.len() == 0 {
            if order.bid {
                limit.add_order(order);
                self.bids.push(limit.clone());
                self.bids_limit.insert(price, limit);
            } else {
                limit.add_order(order);
                self.asks.push(limit.clone());

                self.asks_limit.insert(price, limit);
            }
        }
    } // if match not find then order will sits in orderbook
    // limit.add_order(order);
    // self.bids.push(limit.clone());
    // self.bids_limit.insert(price, limit);
}

// limit.add_order(order);
// self.asks.push(limit.clone());
// self.asks_limit.insert(price, limit);
