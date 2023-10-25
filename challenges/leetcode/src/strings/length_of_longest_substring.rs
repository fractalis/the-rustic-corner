pub fn length_of_longest_substring(s: String) -> i32 {
    let mut substr_hm: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
    let (mut longest, mut start, mut end) = (0, 0, 0);

    for ch in s.chars() {
        let idx = substr_hm.get(&ch);

        match idx {
            Some(idx) => {
                start = *idx;
                substr_hm.clear();
                substr_hm.insert(ch, end);
                longest = std::cmp::max(longest, end - start);
                end += 1;
            }
            None => {
                substr_hm.insert(ch, end);
                end += 1;
            }
        }
    }

    longest as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(length_of_longest_substring("defgdefg".to_string()), 4);
        assert_eq!(length_of_longest_substring("".to_string()), 0);
    }
}
