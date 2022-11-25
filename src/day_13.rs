#[allow(dead_code)]
pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
    use std::collections::VecDeque;
    const MOD: i32 = 1000000007;

    fn sum_subarray_iter(arr: &Vec<i32>) -> i32 {
        let mut monotonic_stack = VecDeque::new();
        let mut sum_of_minimums = 0;

        for i in 0..=arr.len() {
            while !monotonic_stack.is_empty()
                && (i as usize == arr.len()
                    || arr[*monotonic_stack.back().unwrap() as usize] >= arr[i])
            {
                let mid = monotonic_stack.pop_back().unwrap();
                let left_boundary = if monotonic_stack.is_empty() {
                    -1i32
                } else {
                    *monotonic_stack.back().unwrap()
                };
                let right_boundary = i as i32;

                let count = (mid - left_boundary) * (right_boundary - mid) % MOD;

                sum_of_minimums += (count * arr[mid as usize]) % MOD;
                sum_of_minimums %= MOD;
            }
            monotonic_stack.push_back(i as i32);
        }

        sum_of_minimums
    }

    fn sum_subarray_dp(arr: &Vec<i32>) -> i32 {
        let mut dp = vec![0; arr.len()];
        let mut stack = VecDeque::new();

        for i in 0..arr.len() {
            while !stack.is_empty() && arr[*stack.back().unwrap() as usize] >= arr[i] {
                stack.pop_back();
            }

            if stack.is_empty() {
                dp[i] = (i + 1) as i32 * arr[i];
            } else {
                let j = *stack.back().unwrap() as i32;
                dp[i] = dp[j as usize] + (i as i32 - j) * arr[i];
            }

            stack.push_back(i as i32);
        }

        dp.iter().fold(0, |acc, v| (acc + v) % MOD) % MOD
    }
    println!("{}", sum_subarray_iter(&arr));
    sum_subarray_dp(&arr)
}

#[allow(dead_code)]
pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    fn pivot_array_no_vec_deq(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        use std::cmp::Ordering;
        let mut before_pivot = vec![];
        let mut after_pivot = vec![];

        let mut pivot_pos = 0;

        for num in nums {
            match num.cmp(&pivot) {
                Ordering::Less => before_pivot.push(num),
                Ordering::Equal => {
                    after_pivot.push(0);
                    let mut prev = after_pivot[pivot_pos];
                    for i in pivot_pos..after_pivot.len() - 1 {
                        std::mem::swap(&mut after_pivot[i + 1], &mut prev);
                    }
                    after_pivot[pivot_pos] = pivot;
                    pivot_pos += 1;
                }
                Ordering::Greater => after_pivot.push(num),
            }
        }

        before_pivot.extend(after_pivot);
        before_pivot
    }

    fn pivot_array_vec_deq(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        use std::cmp::Ordering;
        use std::collections::VecDeque;
        let mut before_pivot = vec![];
        let mut after_pivot = VecDeque::new();

        for num in nums {
            match num.cmp(&pivot) {
                Ordering::Less => before_pivot.push(num),
                Ordering::Equal => {
                    after_pivot.push_front(num);
                }
                Ordering::Greater => after_pivot.push_back(num),
            }
        }

        before_pivot.extend(after_pivot);
        before_pivot
    }

    fn pivot_array_count_pivots(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        use std::cmp::Ordering;
        let mut result = vec![];
        let mut after_pivot = vec![];
        let mut pivot_num = 0;
        for num in nums {
            match num.cmp(&pivot) {
                Ordering::Less => result.push(num),
                Ordering::Equal => pivot_num += 1,
                Ordering::Greater => after_pivot.push(num),
            }
        }
        for _ in 0..pivot_num {
            result.push(pivot);
        }

        result.extend(after_pivot);
        result
    }

    pivot_array_count_pivots(nums, pivot)
}

#[cfg(test)]
mod test {
    use crate::day_13::*;

    #[test]
    fn test75() {
        println!("{:?}", sum_subarray_mins(vec![2, 2, 2]));
    }

    #[test]
    fn test76() {
        println!("{:?}", sum_subarray_mins(vec![2, 2, 2]));
    }
    #[test]
    fn test77() {
        println!("{:?}", pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10)); //[9,5,3,10,10,12,14]
        println!("{:?}", pivot_array(vec![-3, 4, 3, 2], 2)) //[-3,2,4,3]
    }
}
