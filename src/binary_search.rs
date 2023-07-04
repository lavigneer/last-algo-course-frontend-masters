pub fn binary_search(haystack: &[u32], needle: u32) -> bool {
    if haystack.is_empty() {
        return false;
    }
    let mid = haystack.len() / 2;
    if haystack[mid] == needle {
        return true;
    }
    if haystack[mid] > needle {
        return binary_search(&haystack[..mid], needle);
    }
    binary_search(&haystack[mid + 1..], needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let foo = vec![1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];
        assert!(binary_search(&foo, 69));
        assert!(!binary_search(&foo, 1336));
        assert!(binary_search(&foo, 69420));
        assert!(!binary_search(&foo, 69421));
        assert!(binary_search(&foo, 1));
        assert!(!binary_search(&foo, 0));
    }
}
