pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    use std::collections::{HashMap, HashSet};
    let mut m = HashMap::new();
    let mut s = HashSet::new();

    for x in arr.iter() {
        m.entry(x).or_insert(1);
        *m.get_mut(&x).unwrap() += 1;
    }
    for v in m.values() {
        s.insert(v);
    }
    m.keys().len() == s.len()
}

#[cfg(test)]
mod test {
    use crate::day_18::unique_occurrences;

    #[test]
    fn test89() {
        println!("{}", unique_occurrences(vec![1, 2, 2, 1, 1, 3]));
    }
}
