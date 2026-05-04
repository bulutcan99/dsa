use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt::Debug;
use std::time::Instant;

fn main() {
    // 1. Veri Hazırlığı
    let n = 10_000_000;
    println!("--- Veri seti hazırlanıyor (n: {}) ---", n);

    let mut data: Vec<i32> = (1..=n).collect();
    let target = 7_654_321; // Aramak istediğin sayı

    // Veriyi karıştırıyoruz (Linear search ve sıralama maliyetini görmek için)
    let mut rng = thread_rng();
    data.shuffle(&mut rng);

    // --- LINEAR SEARCH ---
    println!("\n[Linear Search Başlatılıyor...]");
    let start_ls = Instant::now();
    let result_ls = linear_search(&data, target);
    let duration_ls = start_ls.elapsed();

    match result_ls {
        Some(idx) => println!("✅ Bulundu! İndex: {}, Süre: {:?}", idx, duration_ls),
        None => println!("❌ Bulunamadı. Süre: {:?}", duration_ls),
    }

    // --- BINARY SEARCH (SIRALAMA DAHİL) ---
    // Not: Binary search için veri mutlaka sıralı olmalı.
    // Önce sıralama süresini, sonra arama süresini ayrı ayrı ölçüyoruz.
    println!("\n[Binary Search Başlatılıyor (Veri önce sıralanıyor)...]");

    let sort_start = Instant::now();
    data.sort();
    let sort_duration = sort_start.elapsed();
    println!("Sıralama (sort) işlemi tamamlandı: {:?}", sort_duration);

    let start_bs = Instant::now();
    let result_bs = binary_search(&data, target);
    let duration_bs = start_bs.elapsed();

    match result_bs {
        Some(idx) => println!("✅ Bulundu! İndex: {}, Süre: {:?}", idx, duration_bs),
        None => println!("❌ Bulunamadı. Süre: {:?}", duration_bs),
    }

    println!("\n--- Özet ---");
    println!("Linear Search Toplam: {:?}", duration_ls);
    println!(
        "Binary Search (Sıralama + Arama): {:?}",
        sort_duration + duration_bs
    );
    println!("Saf Binary Search (Sıralı Veride): {:?}", duration_bs);
}

/// Linear search: Veriyi tek tek gezer.
fn linear_search<T>(list: &[T], target: T) -> Option<usize>
where
    T: PartialEq,
{
    list.iter().position(|x| *x == target)
}

/// # Binary Search (İkili Arama) Algoritması
///
/// ## Tasarım Notları:
/// 1. **Neden `len()`?**: `high` değerini `len() - 1` yerine direkt `len()` başlattık. Bu sayede liste
///    boş olsa bile "0 - 1" işleminden kaynaklı çökme (underflow) riskini önledik ve aralığı
///    yarım-açık (half-open) `[low, high)` şeklinde güvenle kurduk.
///
/// 2. **Neden `middle + 1`?**: `middle` indeksindeki elemanın hedef olmadığını `if` kontrolüyle
///    onayladığımız için, bir sonraki aramada onu tamamen kapsam dışı bırakıyoruz. Ayrıca bu `+1`
///    hamlesi, `low` ve `high` değerlerinin birbirine çok yaklaştığı durumlarda oluşabilecek
///    sonsuz döngüleri engeller.
fn binary_search<T>(list: &[T], target: T) -> Option<usize>
where
    T: PartialOrd + Debug,
{
    let mut low = 0;
    let mut high = list.len();

    while low < high {
        let middle = (low + high) / 2;
        let middle_value = list.get(middle)?;

        if *middle_value == target {
            return Some(middle);
        } else if *middle_value < target {
            low = middle + 1;
        } else if *middle_value > target {
            high = middle;
        }
    }

    None
}
