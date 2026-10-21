// ============================================================================
// REFERENCE IMPLEMENTATION (Naive, for correctness comparison)
// ============================================================================

use crate::interfaces::{OrderBook, Price, Quantity, Side, Update};

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
        panic!("Todo")
    }

    fn get_spread(&self) -> Option<Price> {
        panic!("Todo")
    }

    fn get_best_bid(&self) -> Option<Price> {
        match op.get_best_bid(){
            Some(price) => println!("meilleur bid : {price}"),
            None => println!("Aucun achat"),
        }
    }

    fn get_best_ask(&self) -> Option<Price> {
        panic!(
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
