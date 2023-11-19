use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|w| {
            let first = w.to_lowercase();
            let mut checked_word= first.chars().collect::<Vec<char>>();
            checked_word.sort();
            let second = word.to_lowercase();
            let mut ref_word = second.chars().collect::<Vec<char>>();
            ref_word.sort();

            first != second && checked_word == ref_word
        })
        .map(|w| w.to_owned())
        .collect::<HashSet<&'a str>>()
}
