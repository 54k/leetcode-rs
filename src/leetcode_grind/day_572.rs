// https://leetcode.com/problems/make-sum-divisible-by-p/description/
pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
    let mut need = 0;
    for &n in &nums {
        need = (need + n) % p;
    }
    use std::collections::HashMap;
    let mut last = HashMap::new();
    last.insert(0, -1);
    let mut res = nums.len() as i32;
    let mut cur = 0;
    for i in 0..nums.len() {
        cur = (cur + nums[i]) % p;
        last.insert(cur, i as i32);
        let mut want = (cur - need + p) % p;
        res = res.min(i as i32 - *last.get(&want).unwrap_or(&-(nums.len() as i32)));
    }
    if res < nums.len() as i32 {
        res
    } else {
        -1
    }
}

// https://leetcode.com/problems/count-number-of-bad-pairs/description/
pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
    use std::collections::HashMap;
    let mut counter = HashMap::new();
    let mut ans = nums.len() as i64 * (nums.len() as i64 - 1) / 2;
    for i in 0..nums.len() {
        let mut e = counter.entry(nums[i] - i as i32).or_insert(0);
        ans -= *e;
        *e += 1;
    }
    ans
}
