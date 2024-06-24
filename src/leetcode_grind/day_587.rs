// https://leetcode.com/problems/minimum-number-of-k-consecutive-bit-flips/description
pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut flipped = vec![false; n];
    let mut valid_flips_from_past_window = 0;
    let mut flip_count = 0;
    let k = k as usize;

    for i in 0..n {
        if i >= k {
            if flipped[i - k] {
                valid_flips_from_past_window -= 1;
            }
        }

        if valid_flips_from_past_window % 2 == nums[i] {
            if i + k > n {
                return -1;
            }
            valid_flips_from_past_window += 1;
            flipped[i] = true;
            flip_count += 1;
        }
    }
    flip_count
}
