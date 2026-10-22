use rust_decimal::prelude::*;
use trading_engine::matching_engine::orderbook_2::*;

#[cfg(test)]
pub mod test {
    use rust_decimal::prelude::*;
    use std::time::{SystemTime, UNIX_EPOCH};
    use trading_engine::matching_engine::orderbook_2::{Limit, Order, OrderBook};

    //     fn test_limit() {
    //         let timestamp = SystemTime::now()
    //             .duration_since(UNIX_EPOCH)
    //             .expect("Time went backwards")
    //             .as_secs();
    //         let new_order_1 = Order::new(dec!(90.0), timestamp, true, 1);
    //         let new_order_2 = Order::new(dec!(69.0), timestamp, true, 2);
    //
    //         let mut new_limit = Limit::new(dec!(82000.0));
    //
    //         new_limit.add_order(new_order_1);
    //
    //         new_limit.add_order(new_order_2.clone());
    //         println!("new limit order {:?}", new_limit);
    //
    //         new_limit.delete_order(new_order_2);
    //         println!("new limit order after cacneling order {:?}", new_limit);
    //     }
    #[test]
    fn test_orderbook() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let new_order_buy = Order::new(dec!(90.0), timestamp, true, 1);
        let new_order_sell = Order::new(dec!(69.0), timestamp, false, 2);
        let mut new_orderbook = OrderBook::new();
        new_orderbook.place_order(dec!(6900), new_order_buy);
        new_orderbook.place_order(dec!(138), new_order_sell);
        println!("bids{:?}", new_orderbook.bids);
        println!("asks {:?}", new_orderbook.asks);
    }
}
