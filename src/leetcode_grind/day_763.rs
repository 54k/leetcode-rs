// https://leetcode.com/problems/maximum-beauty-of-an-array-after-applying-operation/description/?envType=daily-question&envId=2024-12-11
pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
    if nums.len() == 1 {
        return 1;
    }
    let mut max_beauty = 0;
    let mut max_value = 0;

    for &num in &nums {
        max_value = max_value.max(num);
    }

    let mut count = vec![0; max_value as usize + 1];

    for &num in &nums {
        count[(num - k).max(0) as usize] += 1;
        count[(num + k + 1).min(max_value) as usize] -= 1;
    }

    let mut current_sum = 0;
    for val in count {
        current_sum += val;
        max_beauty = max_beauty.max(current_sum);
    }

    max_beauty
}
