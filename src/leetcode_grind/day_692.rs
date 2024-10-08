// https://leetcode.com/problems/minimum-number-of-swaps-to-make-the-string-balanced/description/
pub fn min_swaps(s: String) -> i32 {
    let mut stack_size = 0;
    for ch in s.chars() {
        if ch == '[' {
            stack_size += 1;
        } else {
            if stack_size > 0 {
                stack_size -= 1;
            }
        }
    }
    (stack_size + 1) / 2
}

// https://leetcode.com/problems/maximum-coins-heroes-can-collect/description/?envType=weekly-question&envId=2024-10-08
pub fn maximum_coins(heroes: Vec<i32>, monsters: Vec<i32>, coins: Vec<i32>) -> Vec<i64> {
    let mut m = monsters.into_iter().zip(coins).collect::<Vec<_>>();
    m.sort_by_key(|x| x.0);

    let mut pref = vec![0; m.len() + 1];
    for i in 1..pref.len() {
        pref[i] = pref[i - 1] + m[i - 1].1 as i64;
    }

    let mut ans = vec![0; heroes.len()];
    for i in 0..heroes.len() {
        let hero = heroes[i];
        let mut lo = -1;
        let mut hi = m.len() as i32;
        while lo + 1 < hi {
            let mid = (lo + hi) / 2;
            if m[mid as usize].0 <= hero {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        ans[i] = pref[hi as usize];
    }
    ans
}
