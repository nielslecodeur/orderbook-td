// ============================================================================
// REFERENCE IMPLEMENTATION (Naive, for correctness comparison)
// ============================================================================

use crate::interfaces::{OrderBook, Price, Quantity, Side, Update};
use std::collections::BTreeMap;

pub struct OrderBookImpl {
    bids : BTreeMap<Price,Quantity>,
    asks : BTreeMap<Price,Quantity>,
}

impl OrderBookImpl {
    fn levels(&self, side: Side) -> &BTreeMap<Price, Quantity> {
        match side {
            Side::Bid => &self.bids,
            Side::Ask => &self.asks,
        }
    }
}

impl OrderBook for OrderBookImpl {
    fn new() -> Self {
        OrderBookImpl{
            bids : BTreeMap::new(),
            asks : BTreeMap::new(),
        }
    }

    fn apply_update(&mut self, update: Update) {
        let (side, price, quantity) = match update {
            Update::Set { side, price, quantity } => (side, price, quantity),
            Update::Remove { side, price } => (side, price, 0),
        };

        let levels = match side {
            Side::Bid => &mut self.bids,
            Side::Ask => &mut self.asks,
        };

        if quantity == 0 {
            levels.remove(&price);
        } else {
            levels.insert(price, quantity);
        }
    }

    fn get_spread(&self) -> Option<Price> {
        let best_bid = self.get_best_bid();

        match best_bid {
            Some(bid)=>{
                let best_ask = self.get_best_ask();
                match best_ask {
                    Some(ask) =>{
                        Some(ask - bid)
                    }
                    None => None,
                }

            }
            None => None,
        }


        
    }

    fn get_best_bid(&self) -> Option<Price> {
        self.bids.last_key_value().map(|(&price, _)| price)
    }

    fn get_best_ask(&self) -> Option<Price> {
        self.asks.first_key_value().map(|(&price, _)| price)
    }

    fn get_quantity_at(&self, price: Price, side: Side) -> Option<Quantity> {
        self.levels(side).get(&price).copied()
    }

    fn get_top_levels(&self, side: Side, n: usize) -> Vec<(Price, Quantity)> {
        let levels = self.levels(side).iter().map(|(&price, &quantity)| (price, quantity));
        match side {
            Side::Bid => levels.rev().take(n).collect(),
            Side::Ask => levels.take(n).collect(),
        }
    }

    fn get_total_quantity(&self, side: Side) -> Quantity {
        self.levels(side).values().sum()
    }
}
