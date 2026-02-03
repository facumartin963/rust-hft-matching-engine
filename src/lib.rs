use std::collections::BTreeMap;
use rust_decimal::Decimal;
use slab::Slab;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side { Buy, Sell }

#[derive(Debug)]
pub struct Order {
    pub id: u64,
    pub price: Decimal,
    pub quantity: u32,
    pub side: Side,
}

pub struct OrderBook {
    pub orders: Slab<Order>,
    pub bids: BTreeMap<Decimal, Vec<usize>>,
    pub asks: BTreeMap<Decimal, Vec<usize>>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            orders: Slab::with_capacity(100_000),
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn limit_order(&mut self, side: Side, price: Decimal, mut quantity: u32) {
        if side == Side::Buy {
            while quantity > 0 {
                let Some(mut best_ask_entry) = self.asks.first_entry() else { break; };
                let best_ask_price = *best_ask_entry.key();
                if price < best_ask_price { break; }
                let order_ids = best_ask_entry.get_mut();
                while quantity > 0 && !order_ids.is_empty() {
                    let order_idx = order_ids[0];
                    let maker_order = &mut self.orders[order_idx];
                    if maker_order.quantity <= quantity {
                        quantity -= maker_order.quantity;
                        order_ids.remove(0);
                        self.orders.remove(order_idx);
                    } else {
                        maker_order.quantity -= quantity;
                        quantity = 0;
                    }
                }
                if order_ids.is_empty() { best_ask_entry.remove(); }
            }
        } else {
            while quantity > 0 {
                let Some(mut best_bid_entry) = self.bids.last_entry() else { break; };
                let best_bid_price = *best_bid_entry.key();
                if price > best_bid_price { break; }
                let order_ids = best_bid_entry.get_mut();
                while quantity > 0 && !order_ids.is_empty() {
                    let order_idx = order_ids[0];
                    let maker_order = &mut self.orders[order_idx];
                    if maker_order.quantity <= quantity {
                        quantity -= maker_order.quantity;
                        order_ids.remove(0);
                        self.orders.remove(order_idx);
                    } else {
                        maker_order.quantity -= quantity;
                        quantity = 0;
                    }
                }
                if order_ids.is_empty() { best_bid_entry.remove(); }
            }
        }
        if quantity > 0 {
            let entry = self.orders.vacant_entry();
            let order_idx = entry.key();
            entry.insert(Order { id: order_idx as u64, price, quantity, side });
            let book_side = if side == Side::Buy { &mut self.bids } else { &mut self.asks };
            book_side.entry(price).or_insert_with(Vec::new).push(order_idx);
        }
    }
}