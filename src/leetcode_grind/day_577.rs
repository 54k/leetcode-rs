// https://leetcode.com/problems/minimum-increment-to-make-array-unique/description
pub fn min_increment_for_unique(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let mut cnt = [0; 100001];
    for &n in &nums {
        cnt[n as usize] += 1;
    }
    let mut j = 0;
    for i in 0..100001 {
        while cnt[i] != 0 {
            nums[j] = i as i32;
            j += 1;
            cnt[i] -= 1;
        }
    }
    // nums.sort();
    let mut ans = 0;
    for i in 1..nums.len() {
        while nums[i - 1] >= nums[i] {
            nums[i] += 1;
            ans += 1;
        }
        // println!("{:?}", nums);
    }
    ans
}
