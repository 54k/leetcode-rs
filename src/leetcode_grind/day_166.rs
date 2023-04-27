use std::collections::VecDeque;

// https://leetcode.com/problems/bulb-switcher/
// https://leetcode.com/problems/sqrtx/editorial/
pub fn bulb_switch(n: i32) -> i32 {
    extern "C" {
        fn sqrt(_: f64) -> f64;
    }
    unsafe { sqrt(n as f64) as i32 }
}

// https://leetcode.com/problems/sum-of-unique-elements/description/
pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    nums.into_iter()
        .fold(HashMap::new(), |mut acc, v| {
            *acc.entry(v).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .filter_map(|(k, v)| if v == 1 { Some(k) } else { None })
        .sum::<i32>()
}

// https://leetcode.com/problems/find-lucky-integer-in-an-array/
pub fn find_lucky(arr: Vec<i32>) -> i32 {
    use std::collections::BTreeMap;
    arr.into_iter()
        .fold(BTreeMap::new(), |mut acc, v| {
            *acc.entry(v).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .filter_map(|(k, v)| if v == k { Some(k) } else { None })
        .rev()
        .find(|_| true)
        .unwrap_or(-1)
}

// https://leetcode.com/problems/number-of-good-pairs/description/
pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let mut ans = 0;
    for i in 0..nums.len() {
        ans += *map.get(&nums[i]).unwrap_or(&0);
        *map.entry(nums[i]).or_insert(0) += 1;
    }
    ans
}

// https://leetcode.com/problems/implement-stack-using-queues/
struct MyStack {
    q1: VecDeque<i32>,
    q2: VecDeque<i32>,
}
impl MyStack {
    fn new() -> Self {
        Self {
            q1: VecDeque::new(),
            q2: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {}

    fn pop(&self) -> i32 {
        0
    }

    fn top(&self) -> i32 {
        0
    }

    fn empty(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_458() {
        println!("{}", bulb_switch(3)); // 1
    }
}
