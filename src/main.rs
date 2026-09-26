use std::collections::HashMap;
mod matching_engine;
use matching_engine::orderbook::{BidOrAsk, Order, OrderBook};
use matching_engine::trading_engine::{MatchingEngine, TradingPair};

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

    // println!("{:#?}", order_book);

    let trading_pair = TradingPair::new(String::from("USD"), String::from("BTC"));
    let mut matching_engine = MatchingEngine::new();
    matching_engine.add_new_market(trading_pair.clone());
    println!("matching engin {:#?}", matching_engine);

    let btc_order = Order::new(4.0, BidOrAsk::BID);
    // let mut order_book = OrderBook::new();
    // order_book.add_order(60000.0, btc_order);
    matching_engine
        .place_new_limit_order(trading_pair, 90000.0, btc_order)
        .unwrap();

    println!("mathcing engine is {:#?}", matching_engine);
}
