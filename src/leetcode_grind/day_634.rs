// https://leetcode.com/problems/kth-largest-element-in-a-stream/description/?envType=daily-question&envId=2024-08-12
struct KthLargest {
    nums: Vec<i32>,
    k: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, mut nums: Vec<i32>) -> Self {
        nums.sort();
        Self { nums, k: k as usize }
    }
    
    fn add(&mut self, val: i32) -> i32 {
        let idx = self.find(val);
        self.nums.push(val);
        for i in (idx as usize..self.nums.len() - 1).rev() {
            self.nums.swap(i, i + 1);
        }
        self.nums[self.nums.len() - self.k]
    }

    fn find(&self, val: i32) -> i32 {
        let mut lo = -1;
        let mut hi = self.nums.len() as i32;
        while lo + 1 < hi {
            let mid = (lo + hi) / 2;
            let x = self.nums[mid as usize];
            if val < x {
                hi = mid
            } else {
                lo = mid
            }
        }
        hi
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */