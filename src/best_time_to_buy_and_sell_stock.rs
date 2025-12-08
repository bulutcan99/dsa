// 📈 İstenen Problem: Hisse Senedi Alıp Satmak İçin En İyi Zaman (Best Time to Buy and Sell Stock)
// Tek bir alım ve satım işlemi yaparak elde edilebilecek maksimum karı bulun.
// Satış, alımdan sonraki bir günde yapılmalıdır.

use std::cmp;

pub struct Solution;

impl Solution {
    /// Verilen bir tamsayı dizisi (prices), hisse senedinin ardışık günlerdeki fiyatlarını temsil eder.
    /// Dizideki elemanlar için bir gün alım ve sonraki bir gün satım yaparak elde edilebilecek
    /// maksimum karı hesaplar. Eğer kar elde edilemiyorsa (kayıp varsa), 0 döndürülmelidir.
    ///
    /// Çözüm O(n) zaman karmaşıklığı hedeflemelidir.
    ///
    /// # Parametreler
    /// * `prices`: Hisse senedi fiyatlarını içeren tamsayı dizisi.
    ///
    /// # Döndürür
    /// Elde edilebilecek maksimum kar. Kar elde edilemiyorsa 0.
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let mut lowest = i32::MAX;
        let mut max_profit = 0;
        for price in prices {
            lowest = cmp::min(lowest, price);
            let profit = price - lowest;
            max_profit = cmp::max(max_profit, profit);
        }

        max_profit
    }
}

// --- Testler ---

#[cfg(test)]
mod tests {
    use super::*;

    // Testlerin kolay okunabilmesi için bir makro kullanıyoruz.
    macro_rules! test_case {
        ($name:ident, $prices:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let prices = $prices;
                let expected = $expected;

                // Fonksiyonu çalıştırma
                let result = Solution::max_profit(prices.clone());

                assert_eq!(
                    result, expected,
                    "\nInput: prices = {:?}\nExpected: {}\nGot: {}",
                    prices, expected, result
                );
            }
        };
    }

    // Örnek 1: prices = [7,1,5,3,6,4]. Alım 1 (Gün 2), Satım 6 (Gün 5). Kar: 6-1 = 5.
    test_case!(example_1, vec![7, 1, 5, 3, 6, 4], 5);

    // Örnek 2: prices = [7,6,4,3,1]. Fiyatlar sürekli düşüyor. Kar elde edilemez. Max Kar: 0.
    test_case!(example_2, vec![7, 6, 4, 3, 1], 0);

    // Ek Test Senaryosu: Yüksek kar hemen başında.
    test_case!(test_early_profit, vec![2, 4, 1], 2);

    // Ek Test Senaryosu: Dizi boş. (Normalde kısıtlama verilir, ama burada 0 bekleyelim).
    // Not: Boş dizi için bir kısıtlama yoksa, 0 en mantıklı dönüş değeridir.
    test_case!(test_empty, vec![], 0);

    // Ek Test Senaryosu: Yalnızca bir fiyat (Alım/Satım yapılamaz).
    test_case!(test_single_price, vec![5], 0);

    // Ek Test Senaryosu: Tüm fiyatlar aynı.
    test_case!(test_same_prices, vec![10, 10, 10, 10], 0);

    // Ek Test Senaryosu: Maksimum kar sonlarda.
    test_case!(test_late_profit, vec![3, 2, 6, 5, 0, 3], 4);

    // Ek Test Senaryosu: Alım en sonda, satım yapılamaz (kar = 0).
    test_case!(test_no_sell_after_buy, vec![1, 2, 0], 1);
}

fn main() {
    println!(
        "Best Time to Buy and Sell Stock çözümünüzü 'max_profit' metoduna uygulayın ve 'cargo test' ile testleri çalıştırın."
    );

    // Örnek kullanım (isteğe bağlı)
    let prices = vec![7, 1, 5, 3, 6, 4];
    let result = Solution::max_profit(prices);
    println!("Example 1 Result: {}", result);
}
