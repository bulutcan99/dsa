use std::collections::HashMap;
use std::ops::Deref;

// İstenen problem: Tekrarlanan En Yakın İndis (Contains Duplicate II)
// İki aynı değerin indisleri arasındaki farkın en fazla 'k' olup olmadığını kontrol et.

pub struct Solution;

impl Solution {
    /// Verilen bir tamsayı dizisi (nums) ve bir tam sayı 'k' için,
    /// dizide aynı değere sahip iki farklı indis 'i' ve 'j'nin
    /// mutlak farkı |i - j| <= k olacak şekilde var olup olmadığını kontrol eder.
    ///
    /// Çözüm O(n) zaman ve O(n) alan karmaşıklığı hedeflemelidir.
    ///
    /// # Parametreler
    /// * `nums`: Tamsayı dizisi.
    /// * `k`: İndisler arasındaki maksimum izin verilen fark.
    ///
    /// # Döndürür
    /// Şartı sağlayan bir çift bulunursa 'true', aksi takdirde 'false'.
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for (i, num) in nums.into_iter().enumerate() {
            if let Some(index) = map.get(&num) {
                let abs_dif = i.abs_diff(*index);
                if abs_dif <= k as usize {
                    return true;

                }
            }
            map.insert(num, i);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Testlerin kolay okunabilmesi için bir makro kullanıyoruz.
    macro_rules! test_case {
        ($name:ident, $nums:expr, $k:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let nums = $nums;
                let k = $k;
                let expected = $expected;

                // Fonksiyonu çalıştırma
                let result = Solution::contains_nearby_duplicate(nums.clone(), k);

                assert_eq!(
                    result, expected,
                    "\nInput: nums = {:?}, k = {}\nExpected: {}\nGot: {}",
                    nums, k, expected, result
                );
            }
        };
    }

    // Örnek 1: nums = [1,2,3,1], k = 3. |3-0| = 3 <= 3 -> true
    test_case!(example_1, vec![1, 2, 3, 1], 3, true);

    // Örnek 2: nums = [1,0,1,1], k = 1. |2-0|=2, |3-2|=1. 1 <= 1 -> true
    test_case!(example_2, vec![1, 0, 1, 1], 1, true);

    // Örnek 3: nums = [1,2,3,1,2,3], k = 2. En yakın 2'ler |4-1|=3. 3 > 2 -> false
    test_case!(example_3, vec![1, 2, 3, 1, 2, 3], 2, false);

    // Ek Test Senaryosu: Tekrarlanan eleman yok.
    test_case!(test_no_duplicates, vec![1, 2, 3, 4, 5], 1, false);

    // Ek Test Senaryosu: Aynı sayı yan yana (k=0'da bile true olmalı).
    test_case!(test_adjacent, vec![10, 5, 5, 20], 1, true);
}

fn main() {
    println!(
        "Contains Duplicate II çözümünüzü 'contains_nearby_duplicate' metoduna uygulayın ve 'cargo test' ile testleri çalıştırın."
    );

    // Örnek kullanım (isteğe bağlı)
    let nums = vec![1, 2, 3, 1];
    let k = 3;
    let result = Solution::contains_nearby_duplicate(nums, k);
    println!("Example 1 Result: {}", result);
}