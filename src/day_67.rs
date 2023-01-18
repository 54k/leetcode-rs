// https://leetcode.com/problems/maximum-sum-circular-subarray/solutions/2868539/maximum-sum-circular-subarray/
pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
    let mut cur_min = 0;
    let mut min_sum = nums[0];
    let mut cur_max = 0;
    let mut max_sum = nums[0];
    let mut sum = 0;

    for num in nums {
        cur_min = 0.min(cur_min) + num;
        min_sum = min_sum.min(cur_min);

        cur_max = 0.max(cur_max) + num;
        max_sum = max_sum.max(cur_max);

        sum += num;
    }

    if min_sum == sum {
        max_sum
    } else {
        max_sum.max(sum - min_sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test162() {
        println!("{}", max_subarray_sum_circular(vec![5, -3, 5]));
    }
}
