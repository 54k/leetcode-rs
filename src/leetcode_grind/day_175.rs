// https://leetcode.com/problems/number-of-subsequences-that-satisfy-the-given-sum-condition/description/
pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
    const MOD: i64 = 1000000007;
    nums.sort();
    let mut power = vec![1; nums.len()];
    for i in 1..nums.len() {
        power[i] = (power[i - 1] * 2) as i64 % MOD;
    }
    let (mut i, mut j) = (0, nums.len() as i32 - 1);
    let mut ans = 0;
    while i <= j {
        if nums[i as usize] + nums[j as usize] <= target {
            ans += power[j as usize - i as usize];
            ans %= MOD;
            i += 1;
        } else {
            j -= 1;
        }
    }
    ans as i32
}

// https://leetcode.com/problems/contains-duplicate-ii/description/
pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    for i in 0..nums.len() {
        if set.contains(&nums[i]) {
            return true;
        }
        set.insert(nums[i]);
        if set.len() > k as usize {
            set.remove(&nums[i - k as usize]);
        }
    }
    false
}

// https://leetcode.com/problems/logger-rate-limiter/description/
use std::collections::HashMap;
struct Logger {
    map: HashMap<String, i32>,
}
impl Logger {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        if !self.map.contains_key(&message) || self.map[&message] + 10 <= timestamp {
            self.map.insert(message, timestamp);
            return true;
        }
        false
    }
}

// https://leetcode.com/problems/group-shifted-strings/description/
pub fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    fn find_hash(s: String) -> String {
        let mut hash_key = String::new();
        let s = s.chars().collect::<Vec<char>>();
        for i in 1..s.len() {
            let h = ((s[i] as u8 - s[i - 1] as u8 + 26) % 26 + 'a' as u8);
            hash_key.push(h as char);
        }
        hash_key
    }
    for s in strings {
        let hash = find_hash(s.clone());
        map.entry(hash).or_insert(vec![]).push(s);
    }
    map.into_iter().fold(vec![], |mut acc, (_, v)| {
        acc.push(v);
        acc
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test491() {
        println!("{}", num_subseq(vec![1], 1));
    }
}
