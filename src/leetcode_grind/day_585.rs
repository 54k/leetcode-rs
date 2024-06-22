// https://leetcode.com/problems/count-substrings-without-repeating-character/description
pub fn number_of_special_substrings(s: String) -> i32 {
    let mut ans = 0;
    let mut freq = vec![0; 26];
    let s = s.chars().collect::<Vec<_>>();
    let mut start = 0;
    for end in 0..s.len() {
        freq[s[end] as usize - 'a' as usize] += 1;
        while freq[s[end] as usize - 'a' as usize] > 1 {
            freq[s[start] as usize - 'a' as usize] -= 1;
            start += 1;
        }
        ans += end - start + 1;
    }
    ans as i32
}

// https://leetcode.com/problems/count-number-of-nice-subarrays/description
pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    fn at_most(nums: &Vec<i32>, k: i32) -> i32 {
        let mut window_size = 0;
        let mut subarrays = 0;
        let mut start = 0;

        for end in 0..nums.len() {
            window_size += nums[end] % 2;
            while window_size > k {
                window_size -= nums[start] % 2;
                start += 1;
            }
            subarrays += end - start + 1;
        }
        subarrays as i32
    }

    at_most(&nums, k) - at_most(&nums, k - 1)
}
