use trading_engine::matching_engine::orderbook_2::*;

#[cfg(test)]
pub mod test {
    use std::time::{SystemTime, UNIX_EPOCH};
    use trading_engine::matching_engine::orderbook_2::{Limit, Order};

    #[test]
    fn test_limit() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let new_order_1 = Order::new(90.0, timestamp, true, 1);
        let new_order_2 = Order::new(69.0, timestamp, true, 2);

        let mut new_limit = Limit::new(82000.0);

        new_limit.add_order(new_order_1);

        new_limit.add_order(new_order_2.clone());
        println!("new limit order {:?}", new_limit);

        new_limit.delete_order(new_order_2);
        println!("new limit order after cacneling order {:?}", new_limit);
    }
}
