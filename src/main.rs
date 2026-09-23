#[derive(Debug)]
enum BidOrAsk {
    BID,
    ASK,
}
#[derive(Debug)]
struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    fn new(price: f64) -> Self {
        let scalar = 1000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Price {
            integral,
            fractional,
            scalar,
        }
    }
}

#[derive(Debug)]
struct Limit {
    price: Price,
    orders: Vec<Order>,
}

impl Limit {
    fn new(price: f64) -> Self {
        Limit {
            price: Price::new(price),
            orders: Vec::new(),
        }
    }

    fn add(&mut self, order: Order) {
        self.orders.push(order)
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
    println!("Hello, world!");
    let mut limit = Limit::new(4000.0);
    let order = Order::new(4.5, BidOrAsk::BID);
    limit.add(order);
    println!("limit order  at bid size is {:?}", limit);
}
