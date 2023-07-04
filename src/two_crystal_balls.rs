pub fn two_crystal_balls(breaks: &[bool]) -> Option<usize> {
    let breaks_sqrt = (breaks.len() as f64).sqrt() as usize;
    let mut breaks_iter = breaks.chunks(breaks_sqrt).enumerate();

    let mut prev = breaks_iter.next().unwrap_or((0, &[]));
    for item in breaks_iter {
        if let Some(i) = item.1.first() {
            if i == &true {
                break;
            }
        }
        prev = item
    }
    for i in 0..prev.1.len() {
        if let Some(&true) = prev.1.get(i) {
            return Some(i + (prev.0 * breaks_sqrt));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn it_works() {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..10000);
        let data = [vec![false; idx], vec![true; 10000 - idx]].concat();
        assert_eq!(two_crystal_balls(&data), Some(idx));
        assert_eq!(two_crystal_balls(&vec![false; 821]), None);
    }
}
