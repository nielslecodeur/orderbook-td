// ============================================================================
// REFERENCE IMPLEMENTATION (Naive, for correctness comparison)
// ============================================================================

use crate::interfaces::{OrderBook, Price, Quantity, Side, Update};
use std::collections::BTreeMap;

pub struct OrderBookImpl {
    bids : BTreeMap<Price,Quantity>,
    asks : BTreeMap<Price,Quantity>,
}

impl OrderBook for OrderBookImpl {
    fn new() -> Self {
        OrderBookImpl{
            bids : BTreeMap::new(),
            asks : BTreeMap::new(),
        }
    }

    fn apply_update(&mut self, update: Update) {
        
            
                

            match update {
                Update::Set { price, quantity, side } => {
                    match side {
                        Side::Bid => {
                            self.bids.insert(price, quantity);
                        }
                        Side::Ask => {
                            self.asks.insert(price, quantity);
                        }
                        
                }
                Update::Remove { price, side } => {
        // À compléter
                    Side::quantity =>{
                        
                    }
                }
}
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
        panic!("Todo")
    }

    fn get_top_levels(&self, side: Side, n: usize) -> Vec<(Price, Quantity)> {
        panic!("Todo")
    }

    fn get_total_quantity(&self, side: Side) -> Quantity {
        panic!("Todo")
    }
}
