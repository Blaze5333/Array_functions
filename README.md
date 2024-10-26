# array-functions

A Rust crate providing useful array manipulation utilities and statistical functions for everyday programming needs.

## Features

- Array manipulation utilities
  - Split vectors into chunks
  - Remove duplicates from arrays
  - Find pairs that sum to a target value
- Statistical functions
  - Calculate mean (average)
  - Calculate median
  - Calculate mode

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
array_stat_functions = "0.1.0"  # Replace with your current version
```

## Usage

### Array Utilities

```rust
use array_stat_functions::{chunk_vec, remove_duplicates, find_pairs_with_sum};

fn main() {
    // Chunking a vector
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let chunks = chunk_vec(&numbers, 3).unwrap();
    println!("Chunks: {:?}", chunks);
    // Output: [[1, 2, 3], [4, 5, 6], [7, 8, 9]]

    // Removing duplicates
    let duplicates = vec![1, 2, 3, 2, 4, 3, 5];
    let unique = remove_duplicates(&duplicates);
    println!("Unique elements: {:?}", unique);
    // Output: [1, 2, 3, 4, 5]

    // Finding pairs with sum
    let numbers = vec![1, 5, 7, 1, 5, 3, 2];
    let pairs = find_pairs_with_sum(&numbers, 6);
    println!("Pairs that sum to 6: {:?}", pairs);
    // Output: [(1, 5), (1, 5), (3, 3)]
}
```

### Statistical Functions

```rust
use array_stat_functions::{mean, median, mode};

fn main() {
    let dataset = vec![1.0, 2.0, 3.0, 4.0, 5.0, 5.0];
    
    // Calculate mean
    let avg = mean(&dataset);
    println!("Mean: {}", avg);

    // Calculate median
    let med = median(&dataset);
    println!("Median: {}", med);

    // Calculate mode (works with integers)
    let integers = vec![1, 2, 2, 3, 4, 4, 4, 5];
    let mod_value = mode(&integers);
    println!("Mode: {}", mod_value);
}
```

## API Documentation

### Array Utilities

- `chunk_vec<T: Clone>(vec: &[T], chunk_size: usize) -> Result<Vec<Vec<T>>, &'static str>`
  - Splits a vector into chunks of specified size
  - Returns an error if chunk_size is zero

- `remove_duplicates<T: Clone + PartialEq>(vec: &[T]) -> Vec<T>`
  - Removes duplicate elements while preserving order

- `find_pairs_with_sum<T: Copy + Hash + Eq + Sub<Output = T>>(arr: &[T], target: T) -> Vec<(T, T)>`
  - Finds all pairs of numbers that sum to the target value

### Statistical Functions

- `mean(vec: &[f64]) -> f64`
  - Calculates the arithmetic mean of a vector of numbers

- `median(vec: &[f64]) -> f64`
  - Calculates the median value of a vector of numbers

- `mode(vec: &[i32]) -> i32`
  - Calculates the mode (most frequent value) of a vector of integers

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

This crate was created as a learning project to better understand Rust and contribute to the Rust ecosystem.