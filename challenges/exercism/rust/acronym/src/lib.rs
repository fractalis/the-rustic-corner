pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .filter(|s| !s.is_empty())
        .map(|s| match s {
            s if s.chars().all(|c| c.is_uppercase()) => s.chars().next().unwrap().to_string(),
            s => {
                s[0..1].to_uppercase()
                    + &s[1..]
                        .chars()
                        .filter(|c| c.is_uppercase())
                        .collect::<String>()
            }
        })
        .collect::<String>()
}
