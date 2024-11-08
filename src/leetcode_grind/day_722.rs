// https://leetcode.com/problems/maximum-xor-for-each-query/description/?envType=daily-question&envId=2024-11-08
pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
    let mut prefix_xor = vec![0; nums.len()];
    prefix_xor[0] = nums[0];
    for i in 1..nums.len() {
        prefix_xor[i] = prefix_xor[i - 1] ^ nums[i];
    }
    let mut ans = vec![0; nums.len()];

    let mask = (1 << maximum_bit) - 1;
    for i in 0..nums.len() {
        let current_xor = prefix_xor[prefix_xor.len() - 1 - i];
        ans[i] = current_xor ^ mask;
    }
    ans
}
