// https://leetcode.com/problems/merge-strings-alternately/
pub fn merge_alternately(word1: String, word2: String) -> String {
    let n = word1.len();
    let word1 = word1.chars().collect::<Vec<_>>();
    let m = word2.len();
    let word2 = word2.chars().collect::<Vec<_>>();
    let mut ans = String::new();
    for i in 0..n.max(m) {
        if i < n {
            ans.push(word1[i]);
        }
        if i < m {
            ans.push(word2[i]);
        }
    }
    ans
}

// https://leetcode.com/problems/destroying-asteroids/
pub fn asteroids_destroyed(mass: i32, mut asteroids: Vec<i32>) -> bool {
    asteroids.sort();
    let mut mass = mass as i64;
    for a in asteroids {
        if a as i64 > mass {
            return false;
        }
        mass += a as i64;
    }
    true
}

// https://leetcode.com/problems/partition-array-such-that-maximum-difference-is-k/
pub fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();
    let mut ans = 0;
    let mut x = nums[0];
    for i in 1..nums.len() {
        if nums[i] - x > k {
            ans += 1;
            x = nums[i];
        }
    }
    ans
}

// https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/
pub fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
    use std::collections::HashMap;
    let mut freq = HashMap::new();
    for n in arr {
        *freq.entry(n).or_insert(0) += 1;
    }
    let mut ordered = freq.values().copied().collect::<Vec<_>>();
    ordered.sort();
    ordered.reverse();
    while k > 0 {
        if *ordered.last().unwrap() <= k {
            k -= *ordered.last().unwrap();
            ordered.pop();
        } else {
            break;
        }
    }
    ordered.len() as i32
}

// https://leetcode.com/problems/maximum-69-number/description/
pub fn maximum69_number(num: i32) -> i32 {
    let mut num = num.to_string().chars().collect::<Vec<_>>();
    for i in 0..num.len() {
        if num[i] == '6' {
            num[i] = '9';
            break;
        }
    }
    num.into_iter().collect::<String>().parse().unwrap()
}
