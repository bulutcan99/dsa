// ============================================================================
// PROBLEM 1: Two Sum
// ============================================================================
use std::collections::{HashMap, HashSet};

pub struct Solution1;

impl Solution1 {
    /// Verilen bir tamsayı dizisi (nums) ve hedef (target) için,
    /// toplamı hedefe eşit olan iki sayının indekslerini döndürür.
    ///
    /// # Kısıtlar
    /// - Her input'un tam olarak bir çözümü vardır
    /// - Aynı elemanı iki kez kullanamazsınız
    ///
    /// # Zaman Karmaşıklığı
    /// O(n) - HashMap kullanarak tek pass
    ///
    /// # Alan Karmaşıklığı
    /// O(n) - HashMap için
    ///
    /// # Parametreler
    /// * `nums`: Tamsayı dizisi
    /// * `target`: İstenen toplam
    ///
    /// # Döndürür
    /// Toplamı target'a eşit olan iki indeksin vektörü [i, j]
    pub fn two_sum(nums: &[i32], target: i32) -> Vec<i32> {
        // TODO: Implement this
        vec![]
    }
}

#[cfg(test)]
mod tests_two_sum {
    use super::*;

    macro_rules! test_case {
        ($name:ident, $nums:expr, $target:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let nums = $nums;
                let target = $target;
                let expected = $expected;
                let result = Solution1::two_sum(&nums, target);
                assert_eq!(
                    result, expected,
                    "\nInput: nums = {:?}, target = {}\nExpected: {:?}\nGot: {:?}",
                    nums, target, expected, result
                );
            }
        };
    }

    test_case!(example_1, vec![2, 7, 11, 15], 9, vec![0, 1]);
    test_case!(example_2, vec![3, 2, 4], 6, vec![1, 2]);
    test_case!(example_3, vec![3, 3], 6, vec![0, 1]);
    test_case!(test_negatives, vec![-1, -2, -3, -4, -5], -8, vec![2, 4]);
    test_case!(test_large_numbers, vec![1000000, 2, 3, 999998], 1999998, vec![0, 3]);
}

// ============================================================================
// PROBLEM 2: Valid Palindrome (Kelime için - alphanumeric karakterler)
// ============================================================================

pub struct Solution2;

impl Solution2 {
    /// Verilen bir string'in palindrome olup olmadığını kontrol eder.
    /// Sadece alphanumeric karakterler dikkate alınır, case-insensitive.
    ///
    /// # Açıklama
    /// Bir palindrome, baştan ve sondan okunduğunda aynı olan kelime/cümledir.
    /// Örnek: "racecar", "A man a plan a canal Panama" (boşluklar ve noktalama hariç)
    ///
    /// # Zaman Karmaşıklığı
    /// O(n) - String'i bir kez dolaşırız
    ///
    /// # Alan Karmaşıklığı
    /// O(1) - Two pointers kullanarak, ekstra alan kullanmadan
    ///
    /// # Parametreler
    /// * `s`: Kontrol edilecek string
    ///
    /// # Döndürür
    /// Palindrome ise true, değilse false
    pub fn is_palindrome(s: &str) -> bool {
        // TODO: Implement this
        false
    }
}

#[cfg(test)]
mod tests_palindrome {
    use super::*;

    macro_rules! test_case {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let input = $input;
                let expected = $expected;
                let result = Solution2::is_palindrome(input);
                assert_eq!(
                    result, expected,
                    "\nInput: \"{}\"\nExpected: {}\nGot: {}",
                    input, expected, result
                );
            }
        };
    }

    test_case!(example_1, "A man, a plan, a canal: Panama", true);
    test_case!(example_2, "race a car", false);
    test_case!(example_3, " ", true);
    test_case!(test_single_char, "a", true);
    test_case!(test_empty, "", true);
    test_case!(test_alphanumeric, "0P", false);
    test_case!(test_numbers, "12321", true);
    test_case!(test_mixed, "Was it a car or a cat I saw?", true);
}

// ============================================================================
// PROBLEM 3: Contains Duplicate
// ============================================================================

pub struct Solution3;

impl Solution3 {
    /// Verilen dizide herhangi bir değerin en az iki kez görünüp görünmediğini kontrol eder.
    ///
    /// # Açıklama
    /// Eğer dizide tekrar eden herhangi bir eleman varsa true döndürür.
    ///
    /// # Zaman Karmaşıklığı
    /// O(n) - HashSet kullanarak
    ///
    /// # Alan Karmaşıklığı
    /// O(n) - HashSet için
    ///
    /// # Parametreler
    /// * `nums`: Tamsayı dizisi
    ///
    /// # Döndürür
    /// Duplicate varsa true, yoksa false
    pub fn contains_duplicate(nums: &[i32]) -> bool {
        // TODO: Implement this
        false
    }
}

#[cfg(test)]
mod tests_contains_duplicate {
    use super::*;

    macro_rules! test_case {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let input = $input;
                let expected = $expected;
                let result = Solution3::contains_duplicate(&input);
                assert_eq!(
                    result, expected,
                    "\nInput: {:?}\nExpected: {}\nGot: {}",
                    input, expected, result
                );
            }
        };
    }

    test_case!(example_1, vec![1, 2, 3, 1], true);
    test_case!(example_2, vec![1, 2, 3, 4], false);
    test_case!(example_3, vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2], true);
    test_case!(test_empty, vec![], false);
    test_case!(test_single, vec![1], false);
    test_case!(test_all_same, vec![5, 5, 5, 5], true);
}

// ============================================================================
// PROBLEM 4: Valid Parentheses
// ============================================================================

pub struct Solution4;

impl Solution4 {
    /// Verilen string'deki parantezlerin geçerli olup olmadığını kontrol eder.
    ///
    /// # Açıklama
    /// Bir string geçerlidir eğer:
    /// - Açık parantezler aynı tipte kapalı parantezlerle kapatılmalı
    /// - Açık parantezler doğru sırada kapatılmalı
    /// - Her kapalı parantezin karşılığında aynı tipte açık parantez olmalı
    ///
    /// # Zaman Karmaşıklığı
    /// O(n) - String'i bir kez dolaşırız
    ///
    /// # Alan Karmaşıklığı
    /// O(n) - Stack için worst case
    ///
    /// # Parametreler
    /// * `s`: Sadece '(', ')', '{', '}', '[', ']' karakterlerini içeren string
    ///
    /// # Döndürür
    /// Geçerli parantez dizisi ise true, değilse false
    pub fn is_valid(s: &str) -> bool {
        // TODO: Implement this
        false
    }
}

#[cfg(test)]
mod tests_valid_parentheses {
    use super::*;

    macro_rules! test_case {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let input = $input;
                let expected = $expected;
                let result = Solution4::is_valid(input);
                assert_eq!(
                    result, expected,
                    "\nInput: \"{}\"\nExpected: {}\nGot: {}",
                    input, expected, result
                );
            }
        };
    }

    test_case!(example_1, "()", true);
    test_case!(example_2, "()[]{}", true);
    test_case!(example_3, "(]", false);
    test_case!(example_4, "([)]", false);
    test_case!(example_5, "{[]}", true);
    test_case!(test_empty, "", true);
    test_case!(test_single_open, "(", false);
    test_case!(test_single_close, ")", false);
    test_case!(test_complex, "(([]){()})", true);
    test_case!(test_wrong_order, "]", false);
}

// ============================================================================
// PROBLEM 5: Binary Search
// ============================================================================

pub struct Solution5;

impl Solution5 {
    /// Sorted (artan sırada) bir dizide target değerini arar.
    ///
    /// # Açıklama
    /// Eğer target dizide mevcutsa index'ini döndürür, yoksa -1 döndürür.
    /// Binary search algoritması kullanılmalıdır.
    ///
    /// # Zaman Karmaşıklığı
    /// O(log n) - Her adımda arama alanını yarıya indiririz
    ///
    /// # Alan Karmaşıklığı
    /// O(1) - Iterative yaklaşım kullanırsak
    ///
    /// # Parametreler
    /// * `nums`: Artan sırada sıralanmış unique tamsayı dizisi
    /// * `target`: Aranacak değer
    ///
    /// # Döndürür
    /// Target'ın index'i veya -1 (bulunamadıysa)
    pub fn search(nums: &[i32], target: i32) -> i32 {
        // TODO: Implement this
        -1
    }
}

#[cfg(test)]
mod tests_binary_search {
    use super::*;

    macro_rules! test_case {
        ($name:ident, $nums:expr, $target:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let nums = $nums;
                let target = $target;
                let expected = $expected;
                let result = Solution5::search(&nums, target);
                assert_eq!(
                    result, expected,
                    "\nInput: nums = {:?}, target = {}\nExpected: {}\nGot: {}",
                    nums, target, expected, result
                );
            }
        };
    }

    test_case!(example_1, vec![-1, 0, 3, 5, 9, 12], 9, 4);
    test_case!(example_2, vec![-1, 0, 3, 5, 9, 12], 2, -1);
    test_case!(test_single_found, vec![5], 5, 0);
    test_case!(test_single_not_found, vec![5], -5, -1);
    test_case!(test_first_element, vec![1, 2, 3, 4, 5], 1, 0);
    test_case!(test_last_element, vec![1, 2, 3, 4, 5], 5, 4);
    test_case!(test_empty, vec![], 0, -1);
}

// ============================================================================
// PROBLEM 6: Best Time to Buy and Sell Stock
// ============================================================================

pub struct Solution6;

impl Solution6 {
    /// Bir hisse senedini almak ve satmak için en iyi zamanı bulur.
    ///
    /// # Açıklama
    /// prices[i], i. günde hisse senedinin fiyatıdır.
    /// Maksimum karı elde etmek için bir gün seçip almalı ve
    /// gelecekte farklı bir gün seçip satmalısınız.
    /// Eğer kar elde edilemiyorsa 0 döndürün.
    ///
    /// # Zaman Karmaşıklığı
    /// O(n) - Diziyi bir kez dolaşırız
    ///
    /// # Alan Karmaşıklığı
    /// O(1) - Sadece birkaç değişken kullanırız
    ///
    /// # Parametreler
    /// * `prices`: Her günün hisse fiyatlarını içeren dizi
    ///
    /// # Döndürür
    /// Maksimum kar (kar yoksa 0)
    pub fn max_profit(prices: &[i32]) -> i32 {
        // TODO: Implement this
        0
    }
}

#[cfg(test)]
mod tests_max_profit {
    use super::*;

    macro_rules! test_case {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let input = $input;
                let expected = $expected;
                let result = Solution6::max_profit(&input);
                assert_eq!(
                    result, expected,
                    "\nInput: {:?}\nExpected: {}\nGot: {}",
                    input, expected, result
                );
            }
        };
    }

    test_case!(example_1, vec![7, 1, 5, 3, 6, 4], 5);
    test_case!(example_2, vec![7, 6, 4, 3, 1], 0);
    test_case!(test_single, vec![1], 0);
    test_case!(test_two_increasing, vec![1, 5], 4);
    test_case!(test_two_decreasing, vec![5, 1], 0);
    test_case!(test_all_same, vec![3, 3, 3, 3], 0);
    test_case!(test_max_at_end, vec![2, 4, 1, 7], 6);
}

// ============================================================================
// PROBLEM 7: Longest Common Prefix
// ============================================================================

pub struct Solution7;

impl Solution7 {
    /// Bir string dizisindeki en uzun ortak prefix'i bulur.
    ///
    /// # Açıklama
    /// Eğer ortak prefix yoksa boş string döndürür.
    ///
    /// # Zaman Karmaşıklığı
    /// O(S) - S tüm stringlerdeki toplam karakter sayısı
    ///
    /// # Alan Karmaşıklığı
    /// O(1) - Result string hariç ekstra alan kullanmıyoruz
    ///
    /// # Parametreler
    /// * `strs`: String vektörü
    ///
    /// # Döndürür
    /// En uzun ortak prefix (yoksa boş string)
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        // TODO: Implement this
        String::new()
    }
}

#[cfg(test)]
mod tests_longest_common_prefix {
    use super::*;

    macro_rules! test_case {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let input: Vec<String> = $input.iter().map(|s| s.to_string()).collect();
                let expected = $expected;
                let result = Solution7::longest_common_prefix(input.clone());
                assert_eq!(
                    result, expected,
                    "\nInput: {:?}\nExpected: \"{}\"\nGot: \"{}\"",
                    input, expected, result
                );
            }
        };
    }

    test_case!(example_1, vec!["flower", "flow", "flight"], "fl");
    test_case!(example_2, vec!["dog", "racecar", "car"], "");
    test_case!(test_single, vec!["hello"], "hello");
    test_case!(test_empty_array, vec![""], "");
    test_case!(test_all_same, vec!["test", "test", "test"], "test");
    test_case!(test_one_char, vec!["a", "ab", "abc"], "a");
    test_case!(test_no_common, vec!["abc", "def", "ghi"], "");
}

// ============================================================================
// PROBLEM 8: Valid Anagram
// ============================================================================

pub struct Solution8;

impl Solution8 {
    /// İki string'in birbirinin anagram'ı olup olmadığını kontrol eder.
    ///
    /// # Açıklama
    /// Anagram: Harfleri yeniden düzenleyerek başka bir kelime oluşturmak.
    /// Örnek: "anagram" ve "nagaram" birbirinin anagram'ıdır.
    ///
    /// # Zaman Karmaşıklığı
    /// O(n) - Her string'i bir kez dolaşırız
    ///
    /// # Alan Karmaşıklığı
    /// O(1) - Sadece 26 harflik bir array kullanırız (sabit alan)
    ///
    /// # Parametreler
    /// * `s`: Birinci string
    /// * `t`: İkinci string
    ///
    /// # Döndürür
    /// Anagram ise true, değilse false
    pub fn is_anagram(s: &str, t: &str) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut word_count = HashMap::new();
        for c in s.chars() {
            *word_count.entry(c).or_insert(0) += 1
        }

        for c in t.chars() {
            if let Some(v) = word_count.get_mut(&c) {
                *v -= 1;
                if *v == 0 {
                    word_count.remove(&c);
                }
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests_valid_anagram {
    use super::*;

    macro_rules! test_case {
        ($name:ident, $s:expr, $t:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let s = $s;
                let t = $t;
                let expected = $expected;
                let result = Solution8::is_anagram(s, t);
                assert_eq!(
                    result, expected,
                    "\nInput: s = \"{}\", t = \"{}\"\nExpected: {}\nGot: {}",
                    s, t, expected, result
                );
            }
        };
    }

    test_case!(example_1, "anagram", "nagaram", true);
    test_case!(example_2, "rat", "car", false);
    test_case!(test_empty, "", "", true);
    test_case!(test_single, "a", "a", true);
    test_case!(test_different_length, "abc", "abcd", false);
    test_case!(test_repeated_chars, "aabbcc", "abcabc", true);
    test_case!(test_unicode, "你好", "好你", true);
}

// ============================================================================
// PROBLEM 9: FizzBuzz
// ============================================================================

pub struct Solution9;

impl Solution9 {
    /// FizzBuzz problemini çözer.
    ///
    /// # Açıklama
    /// 1'den n'e kadar (dahil) olan sayılar için:
    /// - Sayı 3'e bölünüyorsa: "Fizz"
    /// - Sayı 5'e bölünüyorsa: "Buzz"
    /// - Sayı hem 3'e hem 5'e bölünüyorsa: "FizzBuzz"
    /// - Hiçbirine bölünmüyorsa: sayının string hali
    ///
    /// # Zaman Karmaşıklığı
    /// O(n) - 1'den n'e kadar dolaşırız
    ///
    /// # Alan Karmaşıklığı
    /// O(n) - Result vektörü için
    ///
    /// # Parametreler
    /// * `n`: Üst limit (dahil)
    ///
    /// # Döndürür
    /// FizzBuzz string vektörü
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for i in 1..=n {
            if i % 3 == 0 &&  i % 5 == 0 {
                result.push("FizzBuzz".to_string());
            }else if i % 5 == 0 {
                result.push("Buzz".to_string());
            }else if i % 3 == 0 {
                result.push("Fizz".to_string());
            }else {
                result.push(i.to_string());
            }
        }
        result
    }
}

#[cfg(test)]
mod tests_fizz_buzz {
    use super::*;

    macro_rules! test_case {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let input = $input;
                let expected: Vec<String> = $expected.iter().map(|s| s.to_string()).collect();
                let result = Solution9::fizz_buzz(input);
                assert_eq!(
                    result, expected,
                    "\nInput: n = {}\nExpected: {:?}\nGot: {:?}",
                    input, expected, result
                );
            }
        };
    }

    test_case!(example_1, 3, vec!["1", "2", "Fizz"]);
    test_case!(example_2, 5, vec!["1", "2", "Fizz", "4", "Buzz"]);
    test_case!(
        example_3,
        15,
        vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz"
        ]
    );
    test_case!(test_one, 1, vec!["1"]);
    test_case!(test_fizzbuzz_only, 15, vec![
        "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
        "14", "FizzBuzz"
    ]);
}

// ============================================================================
// MAIN - Örnek kullanım ve test
// ============================================================================

fn main() {
    println!("=== Rust Algorithm Practice ===\n");

    // Two Sum örneği
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    println!("Two Sum: nums = {:?}, target = {}", nums, target);
    println!("Result: {:?}\n", Solution1::two_sum(&nums, target));

    // Valid Palindrome örneği
    let s = "A man, a plan, a canal: Panama";
    println!("Valid Palindrome: \"{}\"", s);
    println!("Result: {}\n", Solution2::is_palindrome(s));

    // Contains Duplicate örneği
    let nums = vec![1, 2, 3, 1];
    println!("Contains Duplicate: {:?}", nums);
    println!("Result: {}\n", Solution3::contains_duplicate(&nums));

    // Valid Parentheses örneği
    let s = "()[]{}";
    println!("Valid Parentheses: \"{}\"", s);
    println!("Result: {}\n", Solution4::is_valid(s));

    // Binary Search örneği
    let nums = vec![-1, 0, 3, 5, 9, 12];
    let target = 9;
    println!("Binary Search: nums = {:?}, target = {}", nums, target);
    println!("Result: {}\n", Solution5::search(&nums, target));

    // Max Profit örneği
    let prices = vec![7, 1, 5, 3, 6, 4];
    println!("Max Profit: prices = {:?}", prices);
    println!("Result: {}\n", Solution6::max_profit(&prices));

    // Longest Common Prefix örneği
    let strs = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    println!("Longest Common Prefix: {:?}", strs);
    println!("Result: \"{}\"\n", Solution7::longest_common_prefix(strs));

    // Valid Anagram örneği
    let s = "anagram";
    let t = "nagaram";
    println!("Valid Anagram: s = \"{}\", t = \"{}\"", s, t);
    println!("Result: {}\n", Solution8::is_anagram(s, t));

    // FizzBuzz örneği
    let n = 15;
    println!("FizzBuzz: n = {}", n);
    println!("Result: {:?}\n", Solution9::fizz_buzz(n));

    println!("=== Tüm testleri çalıştırmak için: cargo test ===");
}