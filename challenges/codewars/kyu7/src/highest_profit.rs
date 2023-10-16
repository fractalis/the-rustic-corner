#[allow(dead_code)]
fn min_max(lst: &[i32]) -> (i32, i32) {
    let min = lst.iter().min().unwrap();
    let max = lst.iter().max().unwrap();

    (*min, *max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_max() {
        assert_eq!(min_max(&[1, 5, 9, 11, 13]), (1, 13));
        assert_eq!(min_max(&[-1, 0, 1, 2, 3]), (-1, 3));
        assert_eq!(min_max(&[1, 1]), (1, 1));
    }
}
