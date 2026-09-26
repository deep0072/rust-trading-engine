use std::collections::HashMap;

#[derive(Debug)]
pub enum BidOrAsk {
    BID,
    ASK,
}

#[derive(Debug)]
pub struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }
    pub fn add_order(&mut self, price: f64, order: Order) {
        let price = Price::new(price);
        match order.bid_or_ask {
            BidOrAsk::ASK => match self.asks.get_mut(&price) {
                Some(limit) => {
                    println!("limit found");

                    limit.add_order(order);
                }

                None => {
                    println!(" limit not found");
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.asks.insert(price, limit);
                }
            },

            BidOrAsk::BID => match self.bids.get_mut(&price) {
                Some(limit) => {
                    println!("limit found");
                    limit.add_order(order);
                }

                None => {
                    println!(" limit not found");
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.bids.insert(price, limit);
                }
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Price {
    integral: u64,
    scalar: u64,
    fraction: u64,
}

impl Price {
    fn new(price: f64) -> Self {
        let scalar = 10000;
        let integral = price as u64;
        let fraction = ((price % 1.0) * integral as f64) as u64;
        Price {
            integral,
            scalar,
            fraction,
        }
    }
}

#[derive(Debug)]
struct Limit {
    price: Price,
    order: Vec<Order>,
}

impl Limit {
    fn new(price: Price) -> Self {
        Limit {
            price,
            order: Vec::new(),
        }
    }

    fn add_order(&mut self, order: Order) {
        self.order.push(order);
    }
}

#[derive(Debug)]
pub struct Order {
    pub size: f64,
    pub bid_or_ask: BidOrAsk,
}

impl Order {
    pub fn new(size: f64, bid_or_ask: BidOrAsk) -> Self {
        Order { size, bid_or_ask }
    }
}
