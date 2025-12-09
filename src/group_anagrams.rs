// 📚 İstenen Problem: Anagramları Gruplandır (Group Anagrams)
// Verilen string dizisindeki kelimeleri anagram gruplarına ayırın.
// Anagram: Aynı karakterleri farklı sırada içeren kelimeler (örn: "eat", "tea", "ate")

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// Verilen bir string dizisini anagram gruplarına ayırır.
    /// Anagramlar aynı karakterleri içeren ama farklı sırada olan kelimelerdir.
    ///
    /// Çözüm O(n * k) zaman karmaşıklığı hedeflemelidir.
    /// (n = kelime sayısı, k = ortalama kelime uzunluğu)
    ///
    /// # Parametreler
    /// * `strs`: Gruplandırılacak string dizisi
    ///
    /// # Döndürür
    /// Anagram gruplarını içeren 2D vector. Grupların sırası önemli değil.
    ///
    /// # Örnek
    /// ```
    /// let strs = vec!["eat".to_string(), "tea".to_string(), "tan".to_string()];
    /// // Sonuç: [["eat","tea"], ["tan"]]
    /// ```
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut res: HashMap<String, Vec<String>> = HashMap::with_capacity(strs.len());

        for word in strs {
            let mut sorted_chars: Vec<char> = word.chars().collect();
            sorted_chars.sort_unstable();
            let sorted_word: String = sorted_chars.into_iter().collect();

            // entry, istenilen value uzerine erisip islem yapmak icin cok temiz bir API
            res.entry(sorted_word).or_insert_with(Vec::new).push(word);
        }

        res.into_values().collect()
    }
}

// --- Testler ---

#[cfg(test)]
mod tests {
    use super::*;

    // Test sonuçlarını karşılaştırmak için yardımcı fonksiyon
    // (sıra önemli olmadığı için her grubu ve grup içindeki kelimeleri sıralıyoruz)
    fn normalize_result(mut result: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for group in &mut result {
            group.sort();
        }
        result.sort();
        result
    }

    macro_rules! test_case {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let input: Vec<String> = $input.iter().map(|s| s.to_string()).collect();
                let expected: Vec<Vec<String>> = $expected
                    .iter()
                    .map(|group| group.iter().map(|s| s.to_string()).collect()) // ✅ Type annotation yok
                    .collect();

                let result = Solution::group_anagrams(input.clone());

                let normalized_result = normalize_result(result);
                let normalized_expected = normalize_result(expected);

                assert_eq!(
                    normalized_result, normalized_expected,
                    "\nInput: {:?}\nExpected: {:?}\nGot: {:?}",
                    input, normalized_expected, normalized_result
                );
            }
        };
    }

    // Örnek 1: Klasik anagram grupları
    test_case!(
        example_1,
        vec!["eat", "tea", "tan", "ate", "nat", "bat"],
        vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]
    );

    // Örnek 2: Tek karakter
    test_case!(example_2, vec!["a"], vec![vec!["a"]]);

    // Örnek 3: Boş string
    test_case!(example_3, vec![""], vec![vec![""]]);

    // Ek Test: Tüm kelimeler aynı anagram grubunda
    test_case!(
        test_all_anagrams,
        vec!["abc", "bca", "cab", "bac"],
        vec![vec!["abc", "bca", "cab", "bac"]]
    );

    // Ek Test: Hiç anagram yok (her kelime farklı)
    test_case!(
        test_no_anagrams,
        vec!["a", "b", "c"],
        vec![vec!["a"], vec!["b"], vec!["c"]]
    );

    // Ek Test: Uzun kelimeler
    test_case!(
        test_long_words,
        vec!["listen", "silent", "hello", "world"],
        vec![vec!["hello"], vec!["world"], vec!["listen", "silent"]]
    );

    // Ek Test: Tekrarlanan kelimeler
    test_case!(
        test_duplicates,
        vec!["eat", "eat", "tea"],
        vec![vec!["eat", "eat", "tea"]]
    );

    // Ek Test: Boş dizi
    test_case!(
        test_empty_array,
        Vec::<&str>::new(),
        Vec::<Vec<&str>>::new()
    );

    // Ek Test: Farklı uzunlukta kelimeler (anagram olamaz)
    test_case!(
        test_different_lengths,
        vec!["a", "aa", "aaa"],
        vec![vec!["a"], vec!["aa"], vec!["aaa"]]
    );
}

fn main() {
    println!("Group Anagrams çözümünüzü 'group_anagrams' metoduna uygulayın ve 'cargo test' ile testleri çalıştırın.");

    // Örnek kullanım
    let strs = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
    let result = Solution::group_anagrams(strs);
    println!("Example 1 Result: {:?}", result);
}
