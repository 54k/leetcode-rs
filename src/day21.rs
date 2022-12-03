pub fn frequency_sort(s: String) -> String {
    use std::collections::{BinaryHeap, HashMap};
    let mut hm = HashMap::new();
    for c in s.chars().into_iter() {
        if let std::collections::hash_map::Entry::Vacant(e) = hm.entry(c) {
            e.insert(1);
        } else {
            *hm.get_mut(&c).unwrap() += 1;
        }
    }

    let mut heap = BinaryHeap::new();
    hm.iter().for_each(|e| heap.push((*e.1, *e.0)));
    let mut res = String::new();
    while !heap.is_empty() {
        let p = heap.pop().unwrap();
        res.push_str(&(0..p.0).into_iter().map(|_| p.1).collect::<String>());
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test94() {
        println!("{}", frequency_sort("Aabb".to_owned()));
    }
}
