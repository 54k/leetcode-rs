mod rmq_fenwick {
    struct Fenwick {
        nums: Vec<i32>,
    }

    impl Fenwick {
        fn new(cap: usize) -> Self {
            Self { nums: vec![0; cap] }
        }
        fn update(&mut self, val: i32, mut idx: i32) {
            while idx < self.nums.len() as i32 {
                self.nums[idx as usize] += val;
                idx += idx & -idx;
            }
        }
        fn query(&self, mut idx: i32) -> i32 {
            let mut sum = 0;
            while idx > 0 {
                sum += self.nums[idx as usize];
                idx -= idx & -idx;
            }
            sum
        }
    }

    struct NumArray {
        fenwick: Fenwick,
        nums: Vec<i32>,
    }

    impl NumArray {
        fn new(nums: Vec<i32>) -> Self {
            let mut fenwick = Fenwick::new(nums.len() + 1);
            for (i, &n) in nums.iter().enumerate() {
                fenwick.update(n, i as i32 + 1);
            }
            Self { fenwick, nums }
        }

        fn update(&mut self, index: i32, val: i32) {
            let diff = val - self.nums[index as usize];
            self.nums[index as usize] = val;
            self.fenwick.update(diff, index + 1);
        }

        fn sum_range(&self, left: i32, right: i32) -> i32 {
            let left_q = self.fenwick.query(left);
            let right_q = self.fenwick.query(right + 1);
            right_q - left_q
        }
    }

    #[test]
    fn test() {
        let mut num_array = NumArray::new(vec![1, 2, 3]);
        println!("{}", num_array.sum_range(0, 2));
        num_array.update(0, 3);
        num_array.update(1, 3);
        println!("{}", num_array.sum_range(0, 2));
    }
}

mod rmq_segment_tree {
    struct SegmentTree {
        nums: Vec<i32>,
        cap: usize,
    }
    impl SegmentTree {
        fn new(cap: usize) -> Self {
            Self {
                nums: vec![0; cap * 2],
                cap,
            }
        }

        fn update(&mut self, val: i32, mut index: usize) {
            index += self.cap;
            self.nums[index] = val;

            while index > 0 {
                index /= 2;
                self.nums[index] = self.nums[index * 2] + self.nums[index * 2 + 1];
            }
        }

        fn query(&self, mut left: usize, mut right: usize) -> i32 {
            let mut sum = 0;

            left += self.cap;
            right += self.cap;
            while left <= right {
                if left % 2 == 1 {
                    sum += self.nums[left];
                    left += 1;
                }
                if right % 2 == 0 {
                    sum += self.nums[right];
                    right -= 1;
                }
                left /= 2;
                right /= 2;
            }
            sum
        }
    }

    struct NumArray {
        seg_tree: SegmentTree,
    }

    impl NumArray {
        fn new(nums: Vec<i32>) -> Self {
            let mut seg_tree = SegmentTree::new(nums.len());
            for (i, n) in nums.into_iter().enumerate() {
                seg_tree.update(n, i);
            }
            Self { seg_tree }
        }

        fn update(&mut self, index: i32, val: i32) {
            self.seg_tree.update(val, index as usize);
        }

        fn sum_range(&self, left: i32, right: i32) -> i32 {
            self.seg_tree.query(left as usize, right as usize)
        }
    }

    #[test]
    fn test_fenwick() {
        let mut num_array = NumArray::new(vec![1, 2, 3]);
        println!("{}", num_array.sum_range(0, 2));
        num_array.update(0, 3);
        num_array.update(1, 3);
        println!("{}", num_array.sum_range(0, 2));
    }
}

mod rmq_sqrt_decomposition {
    struct NumArray {
        nums: Vec<i32>,
        block: Vec<i32>,
        len: usize,
    }

    impl NumArray {
        fn new(nums: Vec<i32>) -> Self {
            let num_of_blocks = (nums.len() as f64).sqrt();
            let len = (nums.len() as f64 / num_of_blocks).ceil() as usize;
            let block = vec![0; len];

            let mut num_arr = Self {
                nums: vec![0; nums.len()],
                block,
                len,
            };
            for (i, n) in nums.into_iter().enumerate() {
                num_arr.update(i as i32, n);
            }
            num_arr
        }

        fn update(&mut self, index: i32, val: i32) {
            let block_idx = index as usize / self.len;
            let prev = self.nums[index as usize];
            self.nums[index as usize] = val;
            self.block[block_idx] += val - prev;
        }

        fn sum_range(&self, left: i32, right: i32) -> i32 {
            let start_block = left as usize / self.len;
            let end_block = right as usize / self.len;
            let mut sum = 0;
            if start_block == end_block {
                for i in left as usize..=right as usize {
                    sum += self.nums[i];
                }
            } else {
                for i in left as usize..=(start_block + 1) * self.len - 1 {
                    sum += self.nums[i];
                }
                for i in start_block + 1..end_block {
                    sum += self.block[i];
                }
                for i in end_block * self.len..=right as usize {
                    sum += self.nums[i];
                }
            }
            sum
        }
    }

    #[test]
    fn test() {
        let mut num_array = NumArray::new(vec![1, 2, 3]);
        println!("{}", num_array.sum_range(0, 2));
        num_array.update(0, 3);
        num_array.update(1, 3);
        println!("{}", num_array.sum_range(0, 2));
    }
}
