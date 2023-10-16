#[allow(dead_code)]
fn are_you_playing_banjo(name: &str) -> String {
    match name.chars().next() {
        Some('r') | Some('R') => {
            format!("{} plays banjo", name)
        }
        _ => {
            format!("{} does not play banjo", name)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_you_playing_banjo() {
        let invalid_names = vec!["Martin", "Ben", "Sara", "Katie", "Laura"];

        for name in invalid_names {
            assert_eq!(
                are_you_playing_banjo(name),
                format!("{} does not play banjo", name)
            );
        }

        let valid_names = vec!["Ricky", "Rachel", "r0cky", "Rolf", "Racquel"];

        for name in valid_names {
            assert_eq!(are_you_playing_banjo(name), format!("{} plays banjo", name));
        }
    }
}
