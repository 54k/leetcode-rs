// https://leetcode.com/problems/number-of-valid-subarrays/description/
pub fn valid_subarrays(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut st = vec![];
    for i in 0..=nums.len() {
        while !st.is_empty() && (i == nums.len() || nums[i] < nums[st[st.len() - 1]]) {
            ans += (i - st.pop().unwrap());
        }
        st.push(i);
    }

    ans as i32
}

// https://leetcode.com/problems/number-of-smooth-descent-periods-of-a-stock/description/
pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
    let mut l = 0;
    let mut ans = 1;
    for r in 1..prices.len() {
        if prices[r - 1] - prices[r] != 1 {
            l = r;
        }
        ans += r - l + 1;
    }
    ans as i64
}
