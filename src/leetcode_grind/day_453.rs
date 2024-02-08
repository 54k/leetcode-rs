// https://leetcode.com/problems/perfect-squares/description
pub fn num_squares(n: i32) -> i32 {
    use std::collections::HashSet;

    let mut squares = vec![];
    let mut i = 1;

    while i * i <= n {
        squares.push(i * i);
        i += 1;
    }

    let mut queue = HashSet::new();
    queue.insert(n);

    let mut lvl = 1;
    while queue.len() > 0 {
        let mut nxt = HashSet::new();
        for v in queue {
            for &s in &squares {
                if v < s {
                    break;
                } else if v == s {
                    return lvl;
                } else {
                    nxt.insert(v - s);
                }
            }
        }
        queue = nxt;
        lvl += 1;
    }

    lvl
}
