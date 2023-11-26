// https://leetcode.com/problems/maximum-sum-with-exactly-k-elements/
pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
    nums.into_iter().max().unwrap() * k + (k - 1) * k / 2
}

// https://leetcode.com/problems/sum-of-absolute-differences-in-a-sorted-array/description/
pub fn get_sum_absolute_differences_1(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut ans = vec![];
    let mut prefix = vec![0; n];
    prefix[0] = nums[0];
    for i in 1..n {
        prefix[i] = prefix[i - 1] + nums[i];
    }
    for i in 0..n {
        let left = prefix[i] - nums[i];
        let right = prefix[n - 1] - prefix[i];
        let left_count = i as i32;
        let right_count = (n - i - 1) as i32;
        let left_total = left_count * nums[i] - left;
        let right_total = right - right_count * nums[i];
        ans.push(left_total + right_total);
    }
    ans
}

pub fn get_sum_absolute_differences_2(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut total_sum = 0;
    for &num in &nums {
        total_sum += num;
    }

    let mut left_sum = 0;
    let mut ans = vec![0; n];

    for i in 0..n {
        let right_sum = total_sum - left_sum - nums[i];

        let left_count = i as i32;
        let right_count = (n - 1 - i) as i32;

        let left_total = left_count * nums[i] - left_sum;
        let right_total = right_sum - right_count * nums[i];

        ans[i] = left_total + right_total;
        left_sum += nums[i];
    }

    ans
}

// https://leetcode.com/problems/special-array-with-x-elements-greater-than-or-equal-x/description/
pub fn special_array(nums: Vec<i32>) -> i32 {
    fn upper_bound(nums: &Vec<i32>, target: i32) -> i32 {
        let mut lo = 0i32;
        let mut hi = nums.len() as i32;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if nums[mid as usize] < target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    }

    let n = nums.len() as i32;
    let mut nums = nums;
    let mut buckets = vec![0; 1001];
    for &num in &nums {
        buckets[num as usize] += 1;
    }

    let mut j = 0;
    for i in 0..buckets.len() {
        while buckets[i] > 0 {
            nums[j] = i as i32;
            buckets[i] -= 1;
            j += 1;
        }
    }

    let mut lo = 1;
    let mut hi = n;
    while lo <= hi {
        let target = (lo + hi) / 2;
        let ub = upper_bound(&nums, target);
        if n - ub == target {
            return target;
        } else if n - ub > target {
            lo = target + 1;
        } else {
            hi = target - 1;
        }
    }
    -1
}

#[test]
fn test_special_array() {
    let res = special_array(vec![2, 3]);
    println!("{res}"); // 2

    let res = special_array(vec![0, 4, 3, 0, 4]);
    println!("{res}"); // 3

    let res = special_array(vec![0, 0]);
    println!("{res}"); // -1
}
