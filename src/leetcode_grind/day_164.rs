pub fn rob(nums: Vec<i32>) -> i32 {
    fn rob_simple(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
        let (mut t1, mut t2) = (0, 0);
        for i in start..=end {
            let temp = t1;
            let current = nums[i];
            t1 = (current + t2).max(t1);
            t2 = temp;
        }
        t1
    }
    if nums.is_empty() {
        0
    } else if nums.len() == 1 {
        nums[0]
    } else {
        rob_simple(&nums, 0, nums.len() - 2).max(rob_simple(&nums, 1, nums.len() - 1))
    }
}

// https://leetcode.com/problems/smallest-number-in-infinite-set/
mod sis_my_solution {
    use std::collections::BinaryHeap;

    pub struct SmallestInfiniteSet {
        min_heap: BinaryHeap<i32>,
        set: Vec<bool>,
    }

    impl SmallestInfiniteSet {
        pub fn new() -> Self {
            Self {
                min_heap: BinaryHeap::from(vec![-1]),
                set: vec![true; 1001],
            }
        }

        pub fn pop_smallest(&mut self) -> i32 {
            let mut pop = self.min_heap.pop().unwrap();
            let ans = -pop;
            self.set[ans as usize] = false;
            while !self.set[-pop as usize + 1] {
                pop -= 1;
            }
            if !self.min_heap.is_empty() && *self.min_heap.peek().unwrap() == pop - 1 {
                return ans;
            }
            self.min_heap.push(pop - 1);
            ans
        }

        pub fn add_back(&mut self, num: i32) {
            if !self.set[num as usize] {
                self.set[num as usize] = true;
                if -(*self.min_heap.peek().unwrap_or(&(-num))) > num {
                    self.min_heap.push(-num);
                }
            }
        }
    }
}

mod sis_leetcode_solution {
    use std::collections::BinaryHeap;
    use std::collections::HashSet;
    pub struct SmallestInfiniteSet {
        heap: BinaryHeap<i32>,
        set: HashSet<i32>,
        current: i32,
    }
    impl SmallestInfiniteSet {
        pub fn new() -> Self {
            Self {
                heap: BinaryHeap::new(),
                set: HashSet::new(),
                current: 1,
            }
        }

        pub fn pop_smallest(&mut self) -> i32 {
            if !self.heap.is_empty() {
                let pop = -self.heap.pop().unwrap();
                self.set.remove(&pop);
                pop
            } else {
                let ans = self.current;
                self.current += 1;
                ans
            }
        }

        pub fn add_back(&mut self, num: i32) {
            if self.current <= num || self.set.contains(&num) {
                return;
            }
            self.set.insert(num);
            self.heap.push(-num);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test465() {
        println!("{}", rob(vec![1, 2, 3, 1])); // 4
    }

    #[test]
    fn test466() {
        // ["SmallestInfiniteSet", "addBack", "popSmallest", "popSmallest", "popSmallest", "addBack", "popSmallest", "popSmallest", "popSmallest"]
        // [[], [2], [], [], [], [1], [], [], []]
        // Output: [null, null, 1, 2, 3, null, 1, 4, 5]

        let mut sis = sis_my_solution::SmallestInfiniteSet::new();
        sis.add_back(2);
        println!("{}", sis.pop_smallest());
        println!("{}", sis.pop_smallest());
        println!("{}", sis.pop_smallest());
        sis.add_back(1);
        println!("{}", sis.pop_smallest());
        println!("{}", sis.pop_smallest());
        println!("{}", sis.pop_smallest());

        let mut sis = sis_leetcode_solution::SmallestInfiniteSet::new();
        sis.add_back(2);
        println!("{}", sis.pop_smallest());
        println!("{}", sis.pop_smallest());
        println!("{}", sis.pop_smallest());
        sis.add_back(1);
        println!("{}", sis.pop_smallest());
        println!("{}", sis.pop_smallest());
        println!("{}", sis.pop_smallest());
    }
}
