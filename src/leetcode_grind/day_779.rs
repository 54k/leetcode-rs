// https://leetcode.com/problems/minimum-number-of-operations-to-move-all-balls-to-each-box/description/?envType=daily-question&envId=2025-01-06
pub fn min_operations(boxes: String) -> Vec<i32> {
    let n = boxes.len();
    let boxes = boxes.chars().collect::<Vec<_>>();
    let mut answer = vec![0; n];

    let mut balls_to_left = 0;
    let mut moves_to_left = 0;

    let mut balls_to_right = 0;
    let mut moves_to_right = 0;

    for i in 0..n {
        answer[i] += moves_to_left;
        balls_to_left += boxes[i] as i32 - '0' as i32;
        moves_to_left += balls_to_left;

        let j = n - 1 - i;
        answer[j] += moves_to_right;
        balls_to_right += boxes[j] as i32 - '0' as i32;
        moves_to_right += balls_to_right;
    }
    answer
}

// https://leetcode.com/problems/online-majority-element-in-subarray/description/?envType=problem-list-v2&envId=binary-indexed-tree
use std::collections::HashMap;
use rand::Rng; // 0.8.5

struct MajorityChecker {
    idx: HashMap<i32, Vec<i32>>,
    arr: Vec<i32>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
        let mut idx = HashMap::new();
        for (i, &v) in arr.iter().enumerate() {
            idx.entry(v).or_insert(vec![]).push(i as i32);
        }
        MajorityChecker {
            idx,
            arr
        }
    }
    
    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        for i in 0..20 {
            let ri = rand::thread_rng().gen_range(left..=right);
            let a = self.arr[ri as usize]; // random pick    
            let l = self.lb(&self.idx[&a], left);
            let r = self.ub(&self.idx[&a], right);
            if r - l >= threshold {
                return a;
            }
        }
        return -1;
    }

    fn lb(&self, arr: &Vec<i32>, t: i32) -> i32 {
        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let mid = (left + right) / 2;

            if arr[mid] < t {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left as i32
    }

    fn ub(&self, arr: &Vec<i32>, t: i32) -> i32 {
        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let mid = (left + right) / 2;

            if arr[mid] <= t {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left as i32
    }
}

/**
 * Your MajorityChecker object will be instantiated and called as such:
 * let obj = MajorityChecker::new(arr);
 * let ret_1: i32 = obj.query(left, right, threshold);
 */