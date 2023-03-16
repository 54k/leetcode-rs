#[allow(dead_code)]
pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
    use std::collections::BinaryHeap;
    let mut h = BinaryHeap::new();

    for pile in piles {
        h.push(pile);
    }

    for _ in 0..k {
        let i = h.pop().unwrap();
        let r = i / 2;
        h.push(i - r);
    }

    let mut ans = 0;
    for hh in h {
        ans += hh;
    }
    ans
}

#[allow(dead_code)]
fn min_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::VecDeque;
    let mut d: VecDeque<(usize, i32)> = VecDeque::with_capacity(nums.len());
    let mut ans = vec![];

    for r in 0..nums.len() {
        while !d.is_empty() && d.back().unwrap().1 >= nums[r] {
            d.pop_back().unwrap();
        }
        d.push_back((r, nums[r]));

        while !d.is_empty() && (d.front().unwrap().0 as i32) < (r as i32 - k + 1) {
            d.pop_front().unwrap();
        }
        if r >= k as usize - 1 {
            let x = d.front().unwrap();
            ans.push(x.1);
        }
    }

    ans
}

#[allow(dead_code)]
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::VecDeque;
    let mut d: VecDeque<(usize, i32)> = VecDeque::with_capacity(nums.len());
    let mut ans = vec![];

    for r in 0..nums.len() {
        while !d.is_empty() && d.back().unwrap().1 <= nums[r] {
            d.pop_back().unwrap();
        }
        d.push_back((r, nums[r]));

        while !d.is_empty() && (d.front().unwrap().0 as i32) < (r as i32 - k + 1) {
            d.pop_front().unwrap();
        }
        if r >= k as usize - 1 {
            let x = d.front().unwrap();
            ans.push(x.1);
        }
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test131() {
        println!("{}", min_stone_sum(vec![5, 4, 9], 2)); // 12
        println!("{}", min_stone_sum(vec![4, 3, 6, 7], 3)); // 12
    }

    #[test]
    fn test132() {
        println!(
            "{:?}",
            min_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3)
        ); // [-1, -3, -3, -3, 3, 3]
        println!("{:?}", min_sliding_window(vec![1], 1));
        println!("{:?}", min_sliding_window(vec![1, 2], 2));
        println!("{:?}", min_sliding_window(vec![1, 2], 1));
    }

    #[test]
    fn test133() {
        println!(
            "{:?}",
            max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3)
        ); // [3,3,5,5,6,7]
        println!("{:?}", max_sliding_window(vec![1], 1));
        println!("{:?}", max_sliding_window(vec![1, 2], 2));
        println!("{:?}", max_sliding_window(vec![1, 2], 1));
    }
}
