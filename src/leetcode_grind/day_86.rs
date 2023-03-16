// Use two pointers to create the new array of 2n elements.
// The first starting at the beginning and the other starting at (n+1)th position.
// Alternate between them and create the new array.

// https://leetcode.com/problems/shuffle-the-array/
// https://leetcode.com/problems/shuffle-the-array/solutions/2973933/shuffle-the-array/?orderBy=most_relevant
pub fn shuffle(mut nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut ans = vec![0; n as usize * 2];
    for i in 0..n as usize {
        ans[i * 2] = nums[i];
        ans[i * 2 + 1] = nums[i + n as usize];
    }
    ans
}

// https://leetcode.com/problems/subarray-sum-equals-k/
pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    fn running(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut sum = 0;
        for i in 0..nums.len() {
            sum = 0;
            for j in i..nums.len() {
                sum += nums[j];
                if sum == k {
                    ans += 1;
                }
            }
        }
        ans
    }
    fn optimized(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut ans = 0;
        let mut sum = 0;
        let mut map = HashMap::new();
        map.insert(0, 1);
        for i in 0..nums.len() {
            sum += nums[i];
            if map.contains_key(&(sum - k)) {
                ans += *map.get(&(sum - k)).unwrap();
            }
            *map.entry(sum).or_insert(0) += 1;
        }
        ans
    }
    optimized(nums, k)
}

// https://leetcode.com/problems/maximum-product-subarray/
pub fn max_product(nums: Vec<i32>) -> i32 {
    fn brute(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            for j in i..nums.len() {
                let mut prod = 1;
                for k in i..=j {
                    prod *= nums[k];
                }
                ans = ans.max(prod);
            }
        }
        ans
    }
    fn optimized(nums: Vec<i32>) -> i32 {
        // store the result that is the max we have found so far
        let mut ans = nums[0];
        // imax/imin stores the max/min product of
        // subarray that ends with the current number A[i]
        let mut min = ans;
        let mut max = ans;
        for i in 1..nums.len() {
            // multiplied by a negative makes big number smaller, small number bigger
            // so we redefine the extremums by swapping them
            if nums[i] < 0 {
                std::mem::swap(&mut min, &mut max);
            }
            // max/min product for the current number is either the current number itself
            // or the max/min by the previous number times the current one
            max = nums[i].max(max * nums[i]);
            min = nums[i].min(min * nums[i]);

            ans = ans.max(max);
        }
        ans
    }
    optimized(nums)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test203() {
        println!("{:?}", shuffle(vec![2, 5, 1, 3, 4, 7], 3)); // [2,3,5,4,1,7]
        println!("{:?}", shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4)); // [1,4,2,3,3,2,4,1]
    }

    #[test]
    fn test204() {
        println!("{:?}", subarray_sum(vec![1, 1, 1], 2)); // 2
        println!("{:?}", subarray_sum(vec![1, 2, 3], 3)); // 2
        println!("{:?}", subarray_sum(vec![-1, -1, 1], 0)); // 1
    }

    #[test]
    fn test205() {
        println!("{}", max_product(vec![2, 3, -2, 4])); // 6
        println!("{}", max_product(vec![-2, 0, -1])); // 0
    }
}
