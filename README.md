# permute

[![crate.io version badge](https://img.shields.io/crates/v/permute)](https://crates.io/crate/permute)
[![docs.rs status badge](https://img.shields.io/docsrs/permute)](https://docs.rs/permute)
[![github actions status badge](https://github.com/noracodes/permute-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/noracodes/permute-rs/)

Generate permutations of a slice in a memory-efficient and deterministic manner, using
[Heap's algorithm](https://en.wikipedia.org/wiki/Heap%27s_algorithm).

For instance, printing all the permutations of the sequence ["red", "green",
"blue"]:

```rust
use permute::permutations_of;

for permutation in permutations_of(&["red", "green", "blue"]) {
    for element in permutation {
        print!("{}, ", element);
    }
    println!("");
}
```

Based on the ordering provided by Heap’s algorithm, it’s guaranteed that this
program will produce:

```text
red, green, blue,
green, red, blue,
blue, red, green,
red, blue, green,
green, blue, red,
blue, green, red,
```

