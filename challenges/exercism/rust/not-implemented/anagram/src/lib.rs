use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|&x| is_anagram(word, x))
        .map(|&x| x)
        .collect()
}

pub fn is_anagram(word: &str, possible_anagram: &str) -> bool {
    let mut word_to_check = word.to_lowercase().chars().collect::<Vec<char>>();
    let mut pa = possible_anagram
        .to_lowercase()
        .chars()
        .collect::<Vec<char>>();

    word_to_check.sort();
    pa.sort();
    word_to_check == pa && word.to_lowercase() != possible_anagram.to_lowercase()
}
