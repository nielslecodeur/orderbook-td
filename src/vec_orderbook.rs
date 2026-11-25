use crate::interfaces::{OrderBook, Price, Quantity, Side, Update};

// Both sides are sorted by ascending price.
#[derive(Default)]
pub struct VecOrderBook {
    bids: Vec<(Price, Quantity)>,
    asks: Vec<(Price, Quantity)>,
}

impl VecOrderBook {
    fn levels(&self, side: Side) -> &[(Price, Quantity)] {
        match side {
            Side::Bid => &self.bids,
            Side::Ask => &self.asks,
        }
    }
}

impl OrderBook for VecOrderBook {
    fn new() -> Self {
        Self::default()
    }

    fn apply_update(&mut self, update: Update) {
        let (side, price, quantity) = match update {
            Update::Set {
                side,
                price,
                quantity,
            } => (side, price, quantity),
            Update::Remove { side, price } => (side, price, 0),
        };
        let levels = match side {
            Side::Bid => &mut self.bids,
            Side::Ask => &mut self.asks,
        };
        match levels.binary_search_by_key(&price, |level| level.0) {
            Ok(index) if quantity == 0 => {
                levels.remove(index);
            }
            Ok(index) => levels[index].1 = quantity,
            Err(index) if quantity > 0 => levels.insert(index, (price, quantity)),
            Err(_) => {}
        }
    }

    fn get_best_bid(&self) -> Option<Price> {
        self.bids.last().map(|level| level.0)
    }

    fn get_best_ask(&self) -> Option<Price> {
        self.asks.first().map(|level| level.0)
    }

    fn get_spread(&self) -> Option<Price> {
        Some(self.get_best_ask()? - self.get_best_bid()?)
    }

    fn get_quantity_at(&self, price: Price, side: Side) -> Option<Quantity> {
        let levels = self.levels(side);
        let index = levels.binary_search_by_key(&price, |level| level.0).ok()?;
        Some(levels[index].1)
    }

    fn get_top_levels(&self, side: Side, n: usize) -> Vec<(Price, Quantity)> {
        match side {
            Side::Bid => self.bids.iter().rev().take(n).copied().collect(),
            Side::Ask => self.asks.iter().take(n).copied().collect(),
        }
    }

    fn get_total_quantity(&self, side: Side) -> Quantity {
        self.levels(side).iter().map(|level| level.1).sum()
    }
}
