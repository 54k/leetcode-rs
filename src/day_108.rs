// https://leetcode.com/problems/create-maximum-number/description/
// https://leetcode.com/problems/create-maximum-number/solutions/77286/short-python-ruby-c/
// https://leetcode.com/problems/create-maximum-number/solutions/77283/share-my-21ms-java-solution-with-comments/?orderBy=most_relevant
pub fn max_number(mut nums1: Vec<i32>, mut nums2: Vec<i32>, k: i32) -> Vec<i32> {
    fn prepare(nums: &mut Vec<i32>, k: usize) -> Vec<i32> {
        let mut drop = nums.len() - k;
        let mut stack = vec![];
        for n in nums.iter() {
            while !stack.is_empty() && drop > 0 && *stack.last().unwrap() < *n {
                stack.pop();
                drop -= 1;
            }
            stack.push(*n);
        }
        nums[0..k].iter().copied().take(k).collect()
    }
    fn merge(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut a = VecDeque::from(a);
        let mut b = VecDeque::from(b);
        let mut merged = vec![];
        while a.len() + b.len() > 0 {
            let n = if a > b { &mut a } else { &mut b };
            merged.push(n.pop_front().unwrap());
        }
        merged
    }
    let k = k as usize;
    let mut ans = vec![];
    let mut k1 = (k as i32 - nums2.len() as i32).max(0) as usize;
    while k1 <= k.min(nums1.len()) {
        let p1 = prepare(&mut nums1, k1);
        let p2 = prepare(&mut nums2, k - k1);
        println!("{:?} \n{:?} \n ***", p1, p2);
        let merged = merge(p1, p2);
        ans = ans.max(merged);
        k1 += 1;
    }
    ans
}

// https://leetcode.com/problems/maximum-swap/description/
pub fn maximum_swap(num: i32) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test305() {
        println!(
            "{:?}",
            max_number(vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5)
        ); // [9,8,6,5,3]
    }

    #[test]
    fn test306() {
        println!("{}", maximum_swap(2736)); // 7236
        println!("{}", maximum_swap(9973)); // 9973
    }
}
