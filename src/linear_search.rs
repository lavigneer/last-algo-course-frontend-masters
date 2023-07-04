pub fn linear_search(haystack: &Vec<u32>, needle: u32) -> bool {
    for i in 0..haystack.len() {
        if haystack[i] == needle {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let foo = vec![1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];
        assert!(linear_search(&foo, 69));
        assert!(!linear_search(&foo, 1336));
        assert!(linear_search(&foo, 69420));
        assert!(!linear_search(&foo, 69421));
        assert!(linear_search(&foo, 1));
        assert!(!linear_search(&foo, 0));
    }
}
