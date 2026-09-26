use std::collections::HashMap;

#[derive(Debug)]
enum BidOrAsk {
    BID,
    ASK,
}

#[derive(Debug)]
struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl OrderBook {
    fn new() -> Self {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    fn add_order(&mut self, price: f64, order: Order) {
        match order.bid_or_ask {
            BidOrAsk::ASK => {
                let price = Price::new(price);
                match self.asks.get_mut(&price) {
                    Some(limit) => {
                        println!(" limit found for add");
                        limit.add_order(order);
                    }

                    None => {
                        println!(" limit not found for zdddkdkd");
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.asks.insert(price, limit);
                    }
                }
            }
            BidOrAsk::BID => {
                let price = Price::new(price);
                match self.bids.get_mut(&price) {
                    Some(limit) => {
                        println!(" limit found");
                        limit.add_order(order);
                    }

                    None => {
                        println!(" limit not  found adding new buy order in orderbook");
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Price {
    scalar: u64,
    integral: u64,
    fraction: u64,
}

impl Price {
    fn new(price: f64) -> Self {
        let scalar = 1000;
        let integral = price as u64;
        let fraction = ((price % 1.0) * scalar as f64) as u64;

        Price {
            scalar,
            integral,
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
struct Order {
    size: f64,
    bid_or_ask: BidOrAsk,
}

impl Order {
    fn new(size: f64, bid_or_ask: BidOrAsk) -> Self {
        Order { size, bid_or_ask }
    }
}

fn main() {
    let sell_order = Order::new(2.3, BidOrAsk::ASK);
    let buy_order = Order::new(5.0, BidOrAsk::BID);
    let buy_order_deep = Order::new(3.0, BidOrAsk::BID);
    let mut order_book = OrderBook::new();
    let buy_price = 2645.90;
    let sell_price = 1645.90;
    order_book.add_order(buy_price, buy_order);
    order_book.add_order(sell_price, sell_order);
    order_book.add_order(buy_price, buy_order_deep);

    println!("{:#?}", order_book);
}
