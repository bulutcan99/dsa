mod two_sum_leet_code;

use std::cmp;
use std::fmt::Debug;
use std::time::Instant;
use rand::rng;
use rand::seq::SliceRandom;

fn main() {
    // 10 milyonluk sırasız bir veri seti oluşturuluyor
    let mut data: Vec<i32> = (1..=100_000).collect();
    data.shuffle(&mut rng());
    // Timer
    let start = Instant::now();
    selection_sort(data.as_mut_slice());
    println!("Sorted data: {:?}", data);
    println!("Selection sort took {:?}", start.elapsed());
}

pub fn selection_sort<T: Ord + Debug>(data: &mut [T]) {
    for i in 0..data.len().saturating_sub(1) {
        // i'den sonraki kısımda en küçük elemanın index'ini bul
        let min_index = data[i..]
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(idx, _)| idx + i) // offset ekle (çünkü slice i'den başlıyor)
            .unwrap();

        data.swap(i, min_index);
    }
}