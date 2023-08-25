use std::hash::Hash;

// https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/description/
pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];
    for i in 0..nums.len() {
        let mut smaller = 0;
        for j in 0..nums.len() {
            if i == j {
                continue;
            }
            if nums[j] < nums[i] {
                smaller += 1;
            }
        }
        ans.push(smaller);
    }
    ans
}

// https://leetcode.com/problems/interleaving-string/description/
pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    fn rec(
        s1: &Vec<char>,
        i: usize,
        s2: &Vec<char>,
        j: usize,
        s3: &Vec<char>,
        res: &mut Vec<char>,
        memo: &mut HashMap<(usize, usize), bool>,
    ) -> bool {
        if res == s3 && i == s1.len() && j == s2.len() {
            return true;
        }
        if memo.contains_key(&(i, j)) {
            return memo[&(i, j)];
        }
        let mut result = false;

        if i < s1.len() {
            res.push(s1[i]);
            result |= rec(s1, i + 1, s2, j, s3, res);
            res.pop();
        }
        if j < s2.len() {
            res.push(s2[j]);
            result |= rec(s1, i, s2, j + 1, s3, res);
            res.pop();
        }
        memo.insert((i, j), result);
        result
    }

    let s1 = s1.chars().collect::<Vec<_>>();
    let s2 = s2.chars().collect::<Vec<_>>();
    let s3 = s3.chars().collect::<Vec<_>>();
    use std::collections::HashMap;
    let mut memo = HashMap::new();
    let mut res = vec![];
    rec(&s1, 0, &s2, 0, &s3, &mut res, &mut memo)
}
