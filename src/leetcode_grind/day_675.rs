// https://leetcode.com/problems/k-th-smallest-in-lexicographical-order/description/?envType=daily-question&envId=2024-09-22
pub fn find_kth_number(n: i32, k: i32) -> i64 {
    fn count_steps(n: i64, mut prefix1: i64, mut prefix2: i64) -> i64 {
        let mut steps = 0;
        while prefix1 <= n {
            steps += ((n + 1).min(prefix2) - prefix1);
            prefix1 *= 10;
            prefix2 *= 10;
        }
        steps
    }
    let mut k = k as i64;
    let mut n = n as i64;
    let mut curr = 1;
    k -= 1;
    while k > 0 {
        let step = count_steps(n, curr, curr + 1);
        if step <= k {
            curr += 1;
            k -= step;
        } else {
            curr *= 10;
            k -= 1;
        }
    }
    curr
}
