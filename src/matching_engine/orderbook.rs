#![allow(dead_code)]
use rust_decimal::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub enum BidOrAsk {
    BID,
    ASK,
}

#[derive(Debug)]
pub struct OrderBook {
    asks: HashMap<Decimal, Limit>,
    bids: HashMap<Decimal, Limit>,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub fn fill_market_order(&mut self, market_order: &mut Order) {
        let limits = match market_order.bid_or_ask {
            BidOrAsk::ASK => self.bid_limits(),
            BidOrAsk::BID => self.ask_limits(),
        };

        for limit in limits {
            limit.fill_order(market_order);
            if market_order.is_filled() {
                break;
            }
        }
    }

    fn ask_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits = self.asks.values_mut().collect::<Vec<&mut Limit>>();
        limits.sort_by(|a, b| a.price.cmp(&b.price));
        limits
    }

    fn bid_limits(&mut self) -> Vec<&mut Limit> {
        let mut bid_limits = self.bids.values_mut().collect::<Vec<&mut Limit>>();
        bid_limits.sort_by(|a, b| b.price.cmp(&a.price));
        bid_limits
    }

    pub fn add_order(&mut self, price: Decimal, order: Order) {
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

#[derive(Debug)]
struct Limit {
    price: Decimal,
    order: Vec<Order>,
}

impl Limit {
    fn new(price: Decimal) -> Self {
        Limit {
            price,
            order: Vec::new(),
        }
    }

    fn total_volume(&self) -> f64 {
        return self
            .order
            .iter()
            .map(|order| order.size)
            .reduce(|a, b| a + b)
            .unwrap();
    }

    pub fn fill_order(&mut self, market_order: &mut Order) {
        for limit_order in self.order.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    limit_order.size = 0.0;
                }

                false => {
                    limit_order.size -= market_order.size;
                    market_order.size = 0.0;
                }
            }

            if market_order.is_filled() {
                break;
            }
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

    pub fn is_filled(&self) -> bool {
        self.size == 0.0
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn orderbook_fill_order() {
        let mut order_book = OrderBook::new();
        let ask_order = Order::new(40.0, BidOrAsk::ASK);
        let ask_order2 = Order::new(40.0, BidOrAsk::ASK);
        order_book.add_order(dec!(69.0), ask_order);
        order_book.add_order(dec!(45.0), ask_order2);

        let mut bid_order2 = Order::new(40.0, BidOrAsk::BID);
        order_book.fill_market_order(&mut bid_order2);
        println!("after {:?}", bid_order2);
        println!("orderbook {:?}", order_book);
    }
    #[test]
    fn limit_order_multi_fill() {
        let price = dec!(8000.0);
        let mut limit_order = Limit::new(price);
        let mut buy_limit_order_a = Order::new(500.0, BidOrAsk::BID);
        let mut buy_limit_order_b = Order::new(500.0, BidOrAsk::BID);
        limit_order.add_order(buy_limit_order_a);
        limit_order.add_order(buy_limit_order_b);
        println!("before sell {:?}", limit_order);

        let mut market_sell_order = Order::new(999.0, BidOrAsk::ASK);
        limit_order.fill_order(&mut market_sell_order);
        println!("after sell {:?}", limit_order);
        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit_order.order.get(0).unwrap().is_filled(), true);
        assert_eq!(limit_order.order.get(1).unwrap().is_filled(), false);
    }

    #[test]
    fn limit_order_fill() {
        let price = dec!(1000.0);
        let mut limit = Limit::new(price);
        let buy_order = Order::new(69.0, BidOrAsk::BID);
        limit.add_order(buy_order);

        let mut market_sell_order = Order::new(68.0, BidOrAsk::ASK);
        limit.fill_order(&mut market_sell_order);
        println!("{:?}", limit);
    }

    #[test]
    fn test_total_volume() {
        let price = dec!(900.0);
        let mut limit_order = Limit::new(price);
        let buy_order = Order::new(90000.0, BidOrAsk::BID);
        // assert_eq!(limit_order.total_volume(), 0.0);

        limit_order.add_order(buy_order);
        assert_eq!(limit_order.total_volume(), 90000.0);
        let ask_order = Order::new(80000.0, BidOrAsk::ASK);
    }
}
