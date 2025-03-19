    // https://leetcode.com/problems/minimum-operations-to-make-binary-array-elements-equal-to-one-i/description/?envType=daily-question&envId=2025-03-19
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let k = 3;
        let mut cur_flips = 0;        
        let mut total_flips = 0;

        for i in 0..nums.len() {
            if i >= k && nums[i - k] == 2 {
                cur_flips -= 1;
            }

            if (cur_flips % 2) == nums[i] {
                if i + k > nums.len() {
                    return -1;
                }
                nums[i] = 2;
                cur_flips += 1;
                total_flips += 1;
            }
        }

        total_flips
    }