use std::collections::HashMap;
use std::ops::Deref;

pub struct Solution;

impl Solution {
    /// Verilen bir tamsayı dizisi (nums) ve hedef (target) için,
    /// toplamı hedefe eşit olan iki sayının indekslerini döndürür.
    ///
    /// Çözüm O(n) zaman ve O(n) alan karmaşıklığı hedeflemelidir.
    ///
    /// # Parametreler
    /// * `nums`: Tamsayı dizisi.
    /// * `target`: İstenen toplam.
    ///
    /// # Döndürür
    /// Toplamı target'a eşit olan iki indeksin [i, j] bulunduğu bir vektör.
    /// İndeksler küçükten büyüğe sıralanmalıdır.
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_with_index: HashMap<i32, i32> = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(value) = nums_with_index.get(&complement) {
                return vec![*value, index as i32];
            }
            nums_with_index.insert(*num, index as i32);
        }
        vec![]
    }
}

// --- Test Modülü ---
// `cargo test` komutuyla çalıştırılabilir.
#[cfg(test)]
mod tests {
    use super::*;

    // Testlerin kolay okunabilmesi için bir makro kullanıyoruz.
    macro_rules! test_case {
        ($name:ident, $nums:expr, $target:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let nums = $nums;
                let target = $target;
                let expected = $expected;

                // Fonksiyonu çalıştırma
                let mut result = Solution::two_sum(nums.clone(), target);

                // Sonucun sıralı olmasını garanti etmek için testten önce sıralıyoruz
                // (Eğer uygulamanız küçük indeksin önce gelmesini garantilemiyorsa).
                // Ancak problem tanımı zaten küçük indeksi önce döndürmeyi istiyor,
                // bu yüzden sıralı gelmesini bekleriz.
                // Eğer çözümünüz sıralamayı garanti etmiyorsa bu satırı kullanın:
                // result.sort_unstable();

                assert_eq!(
                    result, expected,
                    "\nInput: nums = {:?}, target = {}\nExpected: {:?}\nGot: {:?}",
                    nums, target, expected, result
                );
            }
        };
    }

    // Örnek 1: nums = [3,4,5,6], target = 7 -> [0, 1]
    test_case!(example_1, vec![3, 4, 5, 6], 7, vec![0, 1]);

    // Örnek 2: nums = [4,5,6], target = 10 -> [0, 2]
    test_case!(example_2, vec![4, 5, 6], 10, vec![0, 2]);

    // Örnek 3: nums = [5,5], target = 10 -> [0, 1]
    test_case!(example_3, vec![5, 5], 10, vec![0, 1]);

    test_case!(example_4, vec![5, 5], 10, vec![0, 1]);

    // Ek Test Senaryosu: Negatif Sayılar
    test_case!(
        test_negatives,
        vec![-1, 2, 7, 11, 15],
        10,
        vec![0, 3] // -1 + 11 = 10
    );

    // Ek Test Senaryosu: Hedef 0
    test_case!(
        test_zero_target,
        vec![10, -5, 5, 20],
        0,
        vec![1, 2] // -5 + 5 = 0
    );
}

fn main() {
    println!(
        "Two Sum çözümünüzü 'two_sum' metoduna uygulayın ve 'cargo test' ile testleri çalıştırın."
    );

    // Örnek kullanım (isteğe bağlı)
    let nums = vec![3, 4, 5, 6];
    let target = 7;
    let result = Solution::two_sum(nums, target);
    println!("Example 1 Result: {:?}", result);
}
