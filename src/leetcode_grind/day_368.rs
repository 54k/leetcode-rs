// https://leetcode.com/problems/maximum-element-after-decreasing-and-rearranging/description/
pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
    let mut arr = arr;
    arr.sort();
    let mut ans = 1;
    for i in 1..arr.len() {
        if arr[i] >= ans + 1 {
            ans += 1;
        }
    }
    ans
}
