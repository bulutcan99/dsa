// 📊 İstenen Problem: En Sık Görünen K Eleman (Top K Frequent Elements)
// Verilen bir tamsayı dizisinde en sık görünen K elemanı döndürün.
// Cevap herhangi bir sırada olabilir.

use std::cmp;
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// Verilen bir tamsayı dizisinde (nums) en sık görünen K elemanı döndürür.
    ///
    /// Çözüm O(n) zaman karmaşıklığı hedeflemelidir.
    /// (n = dizi uzunluğu)
    ///
    /// # Parametreler
    /// * `nums`: Tamsayı dizisi
    /// * `k`: Döndürülecek en sık görünen eleman sayısı
    ///
    /// # Döndürür
    /// En sık görünen K elemanı içeren vektör. Sıralama önemli değil.
    ///
    /// # Örnek
    /// ```
    /// let nums = vec![1,1,1,2,2,3];
    /// let k = 2;
    /// // Sonuç: [1,2] (1 üç kez, 2 iki kez görünüyor)
    /// ```
    ///
    /// # Kısıtlar
    /// * 1 <= nums.length <= 10^5
    /// * -10^4 <= nums[i] <= 10^4
    /// * k, dizideki benzersiz eleman sayısı aralığındadır
    /// * Cevabın benzersiz olduğu garanti edilir
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq = HashMap::new();

        for num in nums {
            *freq.entry(num).or_insert(0) += 1;
        }

        let mut items: Vec<(i32, i32)> = freq.into_iter().collect();

        items.sort_by(|a, b| b.1.cmp(&a.1));

        items
            .into_iter()
            .take(k as usize)
            .map(|(num, _)| num)
            .collect()
    }
}

// --- Test Modülü ---
// `cargo test` komutuyla çalıştırılabilir.
#[cfg(test)]
mod tests {
    use super::*;

    // Helper function: Sonucu normalize et (sıralama önemli değil)
    fn normalize_result(mut result: Vec<i32>) -> Vec<i32> {
        result.sort_unstable();
        result
    }

    // Testlerin kolay okunabilmesi için bir makro kullanıyoruz.
    macro_rules! test_case {
        ($name:ident, $nums:expr, $k:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let nums = $nums;
                let k = $k;
                let mut expected = $expected;

                // Fonksiyonu çalıştırma
                let result = Solution::top_k_frequent(nums.clone(), k);

                // Sonuçları normalize et (sıralama önemli değil)
                let normalized_result = normalize_result(result.clone());
                expected.sort_unstable();

                assert_eq!(
                    normalized_result, expected,
                    "\nInput: nums = {:?}, k = {}\nExpected: {:?}\nGot: {:?}",
                    nums, k, expected, result
                );
            }
        };
    }

    // Örnek 1: nums = [1,1,1,2,2,3], k = 2 -> [1,2]
    // 1 üç kez, 2 iki kez, 3 bir kez görünüyor
    test_case!(example_1, vec![1, 1, 1, 2, 2, 3], 2, vec![1, 2]);

    // Örnek 2: nums = [1], k = 1 -> [1]
    // Tek eleman
    test_case!(example_2, vec![1], 1, vec![1]);

    // Örnek 3: nums = [4,1,-1,2,-1,2,3], k = 2 -> [-1,2]
    // -1 ve 2 ikişer kez görünüyor
    test_case!(example_3, vec![4, 1, -1, 2, -1, 2, 3], 2, vec![-1, 2]);


    // Ek Test: Tüm elemanlar farklı (k=1)
    test_case!(
        test_all_different,
        vec![5, 5, 10, 15, 20],
        1,
        vec![5] // ya da herhangi biri
    );

    // Ek Test: Negatif sayılar
    test_case!(
        test_negatives,
        vec![-5, -5, -5, -1, -1, 0],
        2,
        vec![-5, -1]
    );

    // Ek Test: Büyük frekans farkları
    test_case!(
        test_large_frequency_gap,
        vec![1, 1, 1, 1, 1, 2, 3, 4, 5],
        1,
        vec![1]
    );

    // Ek Test: k = tüm benzersiz elemanlar
    test_case!(
        test_k_equals_unique,
        vec![1, 2, 2, 3, 3, 3],
        3,
        vec![1, 2, 3]
    );

    // Ek Test: Aynı elemanlar tekrar ediyor
    test_case!(
        test_duplicates,
        vec![5, 5, 5, 5, 2, 2, 2],
        2,
        vec![5, 2]
    );

    // Ek Test: Sıfır ve negatif karışık
    test_case!(
        test_mixed_zero,
        vec![0, 0, 0, 1, 1, -1, -1, -1, -1],
        2,
        vec![0, -1]
    );
}

fn main() {
    println!("Top K Frequent Elements çözümünüzü 'top_k_frequent' metoduna uygulayın ve 'cargo test' ile testleri çalıştırın.");

    // Örnek kullanım
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    let result = Solution::top_k_frequent(nums, k);
    println!("Example 1 Result: {:?}", result);
}