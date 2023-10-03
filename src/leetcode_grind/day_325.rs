// https://leetcode.com/problems/range-sum-query-mutable/description/
struct NumArray {
    blocks: Vec<i32>,
    nums: Vec<i32>,
    len: usize,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let num_of_elems = (nums.len() as f64).sqrt();
        let len = (nums.len() as f64 / num_of_elems).ceil() as usize;

        let mut rmq = Self {
            blocks: vec![0; len],
            nums: vec![0; nums.len()],
            len,
        };
        for (i, &n) in nums.iter().enumerate() {
            rmq.update(i as i32, n);
        }
        rmq
    }

    fn update(&mut self, index: i32, val: i32) {
        let block = index as usize / self.len;
        self.blocks[block] = self.blocks[block] - self.nums[index as usize] + val;
        self.nums[index as usize] = val;
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let mut sum = 0;
        let left_block = left as usize / self.len;
        let right_block = right as usize / self.len;

        if left_block == right_block {
            for i in left..=right {
                sum += self.nums[i as usize];
            }
            return sum;
        }

        for i in left as usize..=(left_block + 1) * self.len - 1 {
            sum += self.nums[i];
        }
        for i in left_block + 1..right_block {
            sum += self.blocks[i];
        }
        for i in right_block * self.len..=right as usize {
            sum += self.nums[i];
        }

        sum
    }
}
