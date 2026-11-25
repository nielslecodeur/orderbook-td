use crate::{benchmarks::OrderBookBenchmark, orderbook::OrderBookImpl};
use crate::vec_orderbook::VecOrderBook;

mod benchmarks;
mod interfaces;
mod orderbook;
mod vec_orderbook;

// Objective: Complete the orderbook implementation at ./orderbook.rs and run this file to see how fast it is. Faster implementation wins !

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    const ITERATIONS: usize = 100_000;
    let tree = OrderBookBenchmark::run::<OrderBookImpl>("BTreeMap", ITERATIONS);
    OrderBookBenchmark::print_results(&tree);
    let vec = OrderBookBenchmark::run::<VecOrderBook>("Sorted Vec", ITERATIONS);
    OrderBookBenchmark::print_results(&vec);
}

// ============================================================================
// CORRECTNESS TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use crate::{
        interfaces::{OrderBook, Side, Update},
        orderbook::OrderBookImpl,
        vec_orderbook::VecOrderBook,
    };

    fn test_basic_operations<T: OrderBook>() {
        let mut ob = T::new();

        // Add bids
        ob.apply_update(Update::Set {
            price: 10000,
            quantity: 100,
            side: Side::Bid,
        });
        ob.apply_update(Update::Set {
            price: 9950,
            quantity: 150,
            side: Side::Bid,
        });

        // Add asks
        ob.apply_update(Update::Set {
            price: 10050,
            quantity: 80,
            side: Side::Ask,
        });
        ob.apply_update(Update::Set {
            price: 10100,
            quantity: 120,
            side: Side::Ask,
        });

        assert_eq!(ob.get_best_bid(), Some(10000));
        assert_eq!(ob.get_best_ask(), Some(10050));
        assert_eq!(ob.get_spread(), Some(50));
        assert_eq!(ob.get_quantity_at(10000, Side::Bid), Some(100));
        assert_eq!(ob.get_quantity_at(10050, Side::Ask), Some(80));
        assert_eq!(ob.get_quantity_at(10050, Side::Bid), None);
        assert_eq!(ob.get_top_levels(Side::Bid, 1), vec![(10000, 100)]);
        assert_eq!(ob.get_top_levels(Side::Ask, 3), vec![(10050, 80), (10100, 120)]);
        assert!(ob.get_top_levels(Side::Ask, 0).is_empty());
        assert_eq!(ob.get_total_quantity(Side::Bid), 250);
        assert_eq!(ob.get_total_quantity(Side::Ask), 200);
    }

    fn test_updates_and_removes<T: OrderBook>() {
        let mut ob = T::new();

        ob.apply_update(Update::Set {
            price: 10000,
            quantity: 100,
            side: Side::Bid,
        });
        assert_eq!(ob.get_quantity_at(10000, Side::Bid), Some(100));

        // Update quantity
        ob.apply_update(Update::Set {
            price: 10000,
            quantity: 200,
            side: Side::Bid,
        });
        assert_eq!(ob.get_quantity_at(10000, Side::Bid), Some(200));

        // Remove via zero quantity
        ob.apply_update(Update::Set {
            price: 10000,
            quantity: 0,
            side: Side::Bid,
        });
        assert_eq!(ob.get_quantity_at(10000, Side::Bid), None);

        // Remove via Remove update
        ob.apply_update(Update::Set {
            price: 10000,
            quantity: 100,
            side: Side::Bid,
        });
        ob.apply_update(Update::Remove {
            price: 10000,
            side: Side::Bid,
        });
        assert_eq!(ob.get_quantity_at(10000, Side::Bid), None);
    }

    #[test]
    fn btree_orderbook() {
        test_basic_operations::<OrderBookImpl>();
        test_updates_and_removes::<OrderBookImpl>();
    }

    #[test]
    fn sorted_vec_orderbook() {
        test_basic_operations::<VecOrderBook>();
        test_updates_and_removes::<VecOrderBook>();
    }

    #[test]
    fn middle_level_updates_match_the_reference() {
        let mut tree = OrderBookImpl::new();
        let mut vec = VecOrderBook::new();
        for side in [Side::Bid, Side::Ask] {
            for (price, quantity) in [(30, 3), (10, 1), (20, 2), (20, 4), (20, 0), (20, 0)] {
                let update = Update::Set { price, quantity, side };
                tree.apply_update(update.clone());
                vec.apply_update(update);
                for side in [Side::Bid, Side::Ask] {
                    assert_eq!(vec.get_top_levels(side, 4), tree.get_top_levels(side, 4));
                    assert_eq!(vec.get_total_quantity(side), tree.get_total_quantity(side));
                }
                assert_eq!(vec.get_spread(), tree.get_spread());
            }
        }
    }
}
