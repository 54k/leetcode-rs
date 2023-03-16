// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut bought_at = 0usize;
    for i in 0..prices.len() {
        if prices[bought_at] > prices[i] {
            bought_at = i;
        }
        if prices[i] - prices[bought_at] > ans {
            ans = prices[i] - prices[bought_at];
        }
    }
    ans
}

// https://leetcode.com/problems/sum-of-subarray-ranges/description/
pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
    fn brute_force(nums: Vec<i32>) -> i64 {
        let mut ans = 0i64;
        for i in 0..nums.len() {
            for j in i..nums.len() {
                let mut min = i32::MAX;
                let mut max = i32::MIN;
                for k in &nums[i..=j] {
                    min = min.min(*k);
                    max = max.max(*k);
                }
                ans += (max - min) as i64;
            }
        }
        ans
    }

    // Algorithm
    //
    // Initialize an empty stack stack, get the size of nums as n.
    //
    // Iterate over every index from 0 to n (inclusive). For each index right, if either of the following two condition is met:
    //
    // index = n
    // stack is not empty and nums[mid] >= nums[right], where mid is its top value:
    // go to step 3. Otherwise, repeat step 2.
    //
    // Calculate the number of subarrays with nums[mid] as its minimum value:
    //
    // Pop mid from stack.
    // If stack is empty, set left = -1, otherwise, left equals the top element from stack.
    // Increment answer by (right - mid) * (mid - left).
    // Repeat step 2.
    fn stack_solution(nums: Vec<i32>) -> i64 {
        let mut ans = 0;
        let n = nums.len();
        let mut stack = vec![];

        for right in 0..=n {
            while !stack.is_empty() && (right == n || nums[*stack.last().unwrap()] >= nums[right]) {
                let mid = stack.pop().unwrap();
                let left = if stack.is_empty() {
                    -1
                } else {
                    *stack.last().unwrap() as i64
                };
                let mid_num = nums[mid] as i64;
                let mid = mid as i64;
                let right = right as i64;
                let sum = mid_num * (mid - left) * (right - mid);
                ans -= sum;
            }
            stack.push(right);
        }
        stack.clear();

        for right in 0..=n {
            while !stack.is_empty() && (right == n || nums[*stack.last().unwrap()] <= nums[right]) {
                let mid = stack.pop().unwrap();
                let left = if stack.is_empty() {
                    -1
                } else {
                    *stack.last().unwrap() as i64
                };
                let mid_num = nums[mid] as i64;
                let mid = mid as i64;
                let right = right as i64;
                let sum = mid_num * (mid - left) * (right - mid);
                ans += sum;
            }
            stack.push(right);
        }
        ans
    }
    stack_solution(nums)
    // brute_force(nums)
}

// https://leetcode.com/problems/sum-of-subarray-minimums/description/
pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    fn normalize(a: i64) -> i64 {
        a % MOD
    }
    fn mod_add(a: i64, b: i64) -> i64 {
        normalize(normalize(a) + normalize(b))
    }
    fn mod_prod(a: i64, b: i64) -> i64 {
        normalize(normalize(a) * normalize(b))
    }
    let mut ans = 0;
    let mut stack = vec![];
    let n = arr.len();
    for right in 0..=n {
        while !stack.is_empty() && (right == n || (arr[*stack.last().unwrap()] >= arr[right])) {
            let mid = stack.pop().unwrap();
            let mid_el = arr[mid] as i64;
            let mid = mid as i64;
            let left = stack.last().map(|x| *x as i64).unwrap_or_else(|| -1);
            let right = right as i64;
            let subarray_length = (mid - left) * (right - mid);
            ans = mod_add(ans, mod_prod(mid_el, subarray_length));
        }
        stack.push(right);
    }
    ans as i32
}

// https://leetcode.com/problems/largest-rectangle-in-histogram/description/
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut stack = vec![];
    let mut ans = 0;
    for right in 0..=heights.len() {
        while !stack.is_empty()
            && (right == heights.len() || heights[*stack.last().unwrap()] >= heights[right])
        {
            let mid = stack.pop().unwrap();
            let h = heights[mid];
            let left = stack.last().map(|x| *x as i32).unwrap_or(-1);
            let s = h * ((right as i32) - left - 1);
            ans = ans.max(s);
        }
        stack.push(right);
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test296() {
        println!("{}", max_profit(vec![7, 1, 5, 3, 6, 4])); // 5
    }

    #[test]
    fn test297() {
        println!("{}", sub_array_ranges(vec![1, 2, 3])); // 4
        println!("{}", sub_array_ranges(vec![1, 3, 3])); // 4
        println!("{}", sub_array_ranges(vec![4, -2, -3, 4, 1])); // 59
    }

    #[test]
    fn test298() {
        println!("{}", sum_subarray_mins(vec![3, 1, 2, 4])); // 17
        println!("{}", sum_subarray_mins(vec![11, 81, 94, 43, 3])); // 444
    }

    #[test]
    fn test299() {
        println!("{}", largest_rectangle_area(vec![2, 1, 5, 6, 2, 3])); // 10
        println!("{}", largest_rectangle_area(vec![1, 1])); // 2
    }
}
