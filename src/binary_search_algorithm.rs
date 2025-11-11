use std::time::Instant;
use rand::rng;
use rand::seq::SliceRandom;

fn main() {
    // 10 milyonluk sırasız bir veri seti oluşturuluyor
    let mut data: Vec<i32> = (1..=10_000_000).collect();
    data.shuffle(&mut rng());

    let target = 1_234_567;

    // Zaman ölçümü
    let start = Instant::now();
    match linear_search(&data, target) {
        Some(index) => println!("Found {} at index: {} using linear search.", target, index),
        None => println!("{} not found using linear search.", target),
    }
    println!("Linear search took {:?}", start.elapsed());

    // First experiment without sorted
    let start = Instant::now();
    data.sort();
    match binary_search(&data, target) {
        Some(index) => println!("Found {} at index: {} using binary search.", target, index),
        None => println!("{} not found using binary search.", target),
    }
    println!("First Binary search (without sorted) took {:?}", start.elapsed());

    // Second experiment with sorted
    data.sort();
    let start = Instant::now();
    match binary_search(&data, target) {
        Some(index) => println!("Found {} at index: {} using binary search.", target, index),
        None => println!("{} not found using binary search.", target),
    }
    println!("Second Binary search (with sorted) took {:?}", start.elapsed());
}

// Linear search using iter
fn linear_search<T>(list: &[T], target: T) -> Option<usize>
where
    T: PartialEq + Copy,
{
    list.iter().position(|&x| x == target)
}

// Binary search function
fn binary_search<T>(list: &[T], target: T) -> Option<usize>
where
    T: PartialOrd,
{
    let mut first_part = 0;
    let mut last_part = list.len() - 1;

    while first_part <= last_part {
        let middle_part = first_part + (last_part - first_part) / 2;
        let middle = &list[middle_part];

        if middle == &target {
            return Some(middle_part);
        } else if target > *middle {
            first_part = middle_part + 1;
        } else {
            last_part = middle_part - 1;
        }
    }

    None
}

