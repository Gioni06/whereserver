pub fn vec_u16(vec: Vec<u16>) -> Vec<u16> {
    let mut deduplicated = vec.clone();
    deduplicated.sort();
    deduplicated.dedup();
    return deduplicated
}

// add test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_u16() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let deduplicated = vec_u16(vec);
        assert_eq!(deduplicated, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_vec_u16_with_duplicates() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1];
        let deduplicated = vec_u16(vec);
        assert_eq!(deduplicated, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}