// https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/
pub fn max_length(arr: Vec<String>) -> i32 {
    let n = arr.len();
    let mut best = 0;
    let mut mask = 1 << n;

    for m in 1..mask {
        use std::collections::HashSet;
        let mut len = 0;
        let mut set = HashSet::new();

        for i in 0..n {
            if (1 << i) & m != 0 {
                for c in arr[i].chars() {
                    set.insert(c);
                }
                len += arr[i].len();
                if len != set.len() {
                    break;
                }
            }
        }

        if set.len() == len {
            best = best.max(len);
        }
    }

    best as i32
}

// https://leetcode.com/problems/array-of-doubled-pairs/
pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
    use std::collections::HashMap;
    let mut arr = arr;
    let mut cnt = HashMap::new();
    for &x in &arr {
        *cnt.entry(x).or_insert(0) += 1;
    }

    arr.sort_by_key(|x| x.abs());

    for x in arr {
        if *cnt.get(&x).unwrap_or(&0) == 0 {
            continue;
        }
        if *cnt.get(&(2 * x)).unwrap_or(&0) <= 0 {
            return false;
        }

        *cnt.get_mut(&x).unwrap() -= 1;
        *cnt.get_mut(&(2 * x)).unwrap() -= 1;
    }
    true
}
