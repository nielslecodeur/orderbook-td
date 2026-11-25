# High-Performance Order Book With Rust

A competitive programming challenge to build the fastest possible order book data structure in Rust. The goal is to achieve sub-nanosecond operations for critical hot-path functions.

## Comparaison des deux versions

Les deux carnets implémentent le même trait `OrderBook` et passent les mêmes tests.

- **BTreeMap** utilise un arbre ordonné par côté. [Code de la version de référence](src/orderbook.rs)
- **Vec trié** conserve les prix dans l’ordre croissant. Une recherche binaire trouve le niveau à modifier. Le meilleur achat est le dernier élément et la meilleure vente est le premier. [Code de la version Vec](src/vec_orderbook.rs)

```sh
cargo test
cargo run --release
```

Le programme mesure les deux versions sur la même suite d’opérations. Chaque valeur ci-dessous est la médiane des temps moyens de trois exécutions locales, en nanosecondes par appel.

| Opération | BTreeMap | Vec trié |
|---|---:|---:|
| Mise à jour (`Set`) | 9,18 | 8,49 |
| Meilleur achat | 1,35 | 0,63 |
| Meilleure vente | 1,31 | 0,47 |
| Spread | 2,07 | 0,72 |
| Quantité à un prix | 6,48 | 7,50 |

Sur cette charge, le Vec calcule le spread environ 2,9 fois plus vite. La BTreeMap retrouve plus vite la quantité à un prix donné. Le benchmark répète chaque `Set` 1 000 fois et ne mesure aucune suppression. Il mesure donc surtout le remplacement de quantités. Une insertion ou une suppression au milieu du Vec peut déplacer tous les niveaux suivants.

## Objective

Complete the `OrderBook` trait implementation in `src/orderbook.rs` and optimize it for maximum performance. The faster your implementation, the better!

## What is an Order Book?

An order book is a fundamental data structure in financial trading systems that maintains:
- **Bids**: Buy orders sorted by price (highest first)
- **Asks**: Sell orders sorted by price (lowest first)
- **Price levels**: Each price point with its associated quantity

## Project Structure

```
src/
├── main.rs          # Entry point with benchmarks and tests
├── interfaces.rs    # OrderBook trait and type definitions
├── orderbook.rs     # BTreeMap implementation
├── vec_orderbook.rs # Sorted Vec implementation
└── benchmarks.rs    # Comprehensive benchmarking framework
```

## Implementation Requirements

Implement the `OrderBook` trait with the following methods:

### Core Operations (HOT PATH - Optimize heavily!)
- `apply_update(&mut self, update: Update)` - Add/update/remove price levels
- `get_spread(&self) -> Option<Price>` - Calculate bid-ask spread

### Query Operations
- `get_best_bid(&self) -> Option<Price>` - Get highest bid price
- `get_best_ask(&self) -> Option<Price>` - Get lowest ask price
- `get_quantity_at(&self, price: Price, side: Side) -> Option<Quantity>`
- `get_top_levels(&self, side: Side, n: usize) -> Vec<(Price, Quantity)>`
- `get_total_quantity(&self, side: Side) -> Quantity`

## Original exercise instructions

1. **Clone and setup**:
   ```bash
   cargo build --release
   ```

2. **Implement the trait** in `src/orderbook.rs`:
   ```rust
   use std::collections::BTreeMap;

   pub struct OrderBookImpl {
       bids: BTreeMap<Price, Quantity>,
       asks: BTreeMap<Price, Quantity>,
   }

   impl OrderBook for OrderBookImpl {
       fn new() -> Self {
           OrderBookImpl {
               bids: BTreeMap::new(),
               asks: BTreeMap::new(),
           }
       }
       // ... implement other methods
   }
   ```

3. **Run tests** to verify correctness:
   ```bash
   cargo test
   ```

4. **Run benchmarks** to measure performance:
   ```bash
   cargo run --release
   ```

## Benchmark Metrics

The benchmark suite measures:
- **Update operations** (avg, P50, P95, P99)
- **Get best bid/ask** latency
- **Spread calculation** latency
- **Random reads** performance
- **Total operations**: 100,000 iterations

Example output:
```
============================================================
  BENCHMARK RESULTS: OrderBook
============================================================
  Total Operations: 100000
  ---
  Update Operations:
    Average: 45.23 ns
    P50:     42 ns
    P95:     67 ns
    P99:     89 ns
  ---
  Get Best Bid:
    Average: 12.45 ns
  ...
```

## Optimization Tips

1. **Data Structures**: Carefully chose your data structure. It will be the most critical choice

2. **Hot Path Optimization**:
   - Minimize allocations in `apply_update()`
   - Maximize cache usage

3. **Profiling Tools**:
   ```bash
   # Install flamegraph
   cargo install flamegraph

   # Generate flame graph
   cargo flamegraph

   # Run micro-benchmarks
   cargo bench
   ```

4. **Advanced Techniques**:
   - SIMD for batch operations
   - Lock-free data structures
   - Memory pooling for allocations
   - Branch prediction optimization

## Competition Goal

**Achieve sub-nanosecond operations!**

##  Correctness Tests

Two test suites ensure implementation correctness:

1. **Basic Operations**: Tests bid/ask insertion and queries
2. **Updates & Removes**: Tests quantity updates and level removal

All tests must pass before benchmarking.

## Type Definitions

```rust
// Price in units of 10^-4 (e.g., 10000 = 1.0000)
pub type Price = i64;

// Quantity at a price level
pub type Quantity = u64;

// Order side
pub enum Side { Bid, Ask }

// Update operations
pub enum Update {
    Set { price: Price, quantity: Quantity, side: Side },
    Remove { price: Price, side: Side },
}
```

## Contributing

This is a competitive programming challenge. May the fastest implementation win!

---

**Good luck and happy optimizing!**
