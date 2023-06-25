// https://leetcode.com/problems/tallest-billboard/description/
pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
    use std::collections::{HashMap, HashSet};
    let n = rods.len();

    fn subsets(rods: &Vec<i32>, left: usize, right: usize) -> HashMap<i32, i32> {
        let mut states = HashSet::new();
        states.insert((0, 0));

        for i in left..right {
            let r = rods[i];

            let mut new_states = HashSet::new();
            for (k, v) in &states {
                new_states.insert((*k + r, *v));
                new_states.insert((*k, *v + r));
            }

            states.extend(new_states);
        }

        let mut hm = HashMap::new();
        for (l, r) in states {
            let mut e = hm.entry(l - r).or_insert(0);
            *e = (*e).max(l);
        }
        hm
    }

    let left = subsets(&rods, 0, n / 2);
    let right = subsets(&rods, n / 2, n);

    // println!("{:?}", left);
    // println!("{:?}", right);

    let mut ans = 0;
    for &diff in left.keys() {
        if right.contains_key(&(-diff)) {
            ans = ans.max(left[&diff] + right[&(-diff)]);
        }
    }

    ans
}