// https://leetcode.com/problems/fair-distribution-of-cookies/description/
pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
    let mut distribute = vec![0; k as usize];

    fn dfs(
        i: i32,
        distribute: &mut Vec<i32>,
        cookies: &Vec<i32>,
        k: i32,
        mut zero_count: i32,
    ) -> i32 {
        if cookies.len() as i32 - i < zero_count {
            return i32::MAX;
        }

        if i == cookies.len() as i32 {
            let mut unfairness = i32::MIN;
            for value in distribute {
                unfairness = unfairness.max(*value);
            }
            return unfairness;
        }

        let mut answer = i32::MAX;

        for j in 0..k {
            zero_count -= if distribute[j as usize] == 0 { 1 } else { 0 };
            distribute[j as usize] += cookies[i as usize];

            answer = answer.min(dfs(i + 1, distribute, cookies, k, zero_count));

            distribute[j as usize] -= cookies[i as usize];
            zero_count += if distribute[j as usize] == 0 { 1 } else { 0 };
        }

        answer
    }

    dfs(0, &mut distribute, &cookies, k, k)
}

// https://leetcode.com/problems/partition-to-k-equal-sum-subsets/description/
pub fn can_partition_k_subsets_tle(nums: Vec<i32>, k: i32) -> bool {
    let mut subsets = vec![0; k as usize];

    fn dfs(i: i32, nums: &Vec<i32>, subsets: &mut Vec<i32>, k: i32) -> bool {
        if i == nums.len() as i32 {
            for i in 1..subsets.len() {
                if subsets[i as usize - 1] != subsets[i as usize] {
                    return false;
                }
            }
            return true;
        }
        let mut answer = false;
        for j in 0..k {
            subsets[j as usize] += nums[i as usize];
            answer = answer || dfs(i + 1, nums, subsets, k);
            subsets[j as usize] -= nums[i as usize];
        }
        answer
    }

    dfs(0, &nums, &mut subsets, k)
}

pub fn can_partition_k_subsets_backtrack_sort(mut nums: Vec<i32>, k: i32) -> bool {
    fn backtrack(
        arr: &Vec<i32>,
        index: usize,
        count: i32,
        curr_sum: i32,
        k: i32,
        target_sum: i32,
        taken: &mut Vec<bool>,
    ) -> bool {
        let n = arr.len();

        // We made k - 1 subsets with target sum and last subset will also have target sum.
        if count == k - 1 {
            return true;
        }

        // Current subset sum exceeds target sum, no need to proceed further.
        if curr_sum > target_sum {
            return false;
        }

        // When current subset sum reaches target sum then one subset is made.
        // Increment count and reset current subset sum to 0.
        if curr_sum == target_sum {
            return backtrack(arr, 0, count + 1, 0, k, target_sum, taken);
        }

        // Try not picked elements to make some combinations.
        for j in index..n {
            if !taken[j] {
                taken[j] = true;

                // If using current jth element in this subset leads to make all valid subsets.
                if backtrack(arr, j + 1, count, curr_sum + arr[j], k, target_sum, taken) {
                    return true;
                }

                taken[j] = false;
            }
        }

        false
    }

    let mut total_array_sum = 0;
    let n = nums.len();

    for i in 0..n {
        total_array_sum += nums[i];
    }

    if total_array_sum % k != 0 {
        return false;
    }

    nums.sort();
    let nums = nums.into_iter().rev().collect();

    let target_sum = total_array_sum / k;
    let mut taken = vec![false; n];

    backtrack(&nums, 0, 0, 0, k, target_sum, &mut taken)
}

pub fn can_partition_k_subsets_memo_bitmask(mut nums: Vec<i32>, k: i32) -> bool {
    use std::collections::HashMap;

    fn backtrack(
        arr: &Vec<i32>,
        index: usize,
        count: i32,
        curr_sum: i32,
        k: i32,
        target_sum: i32,
        mut mask: i32,
        memo: &mut HashMap<i32, bool>,
    ) -> bool {
        let n = arr.len();

        // We made k - 1 subsets with target sum and last subset will also have target sum.
        if count == k - 1 {
            return true;
        }

        // Current subset sum exceeds target sum, no need to proceed further.
        if curr_sum > target_sum {
            return false;
        }

        if memo.contains_key(&mask) {
            return memo[&mask];
        }

        // When current subset sum reaches target sum then one subset is made.
        // Increment count and reset current subset sum to 0.
        if curr_sum == target_sum {
            let ans = backtrack(arr, 0, count + 1, 0, k, target_sum, mask, memo);
            memo.insert(mask, ans);
            return ans;
        }

        // Try not picked elements to make some combinations.
        for j in index..n {
            if ((mask >> j) & 1) == 0 {
                mask |= 1 << j;

                // If using current jth element in this subset leads to make all valid subsets.
                if backtrack(
                    arr,
                    j + 1,
                    count,
                    curr_sum + arr[j],
                    k,
                    target_sum,
                    mask,
                    memo,
                ) {
                    return true;
                }

                mask ^= 1 << j;
            }
        }

        memo.insert(mask, false);
        false
    }

    let mut total_array_sum = 0;
    let n = nums.len();

    for i in 0..n {
        total_array_sum += nums[i];
    }

    if total_array_sum % k != 0 {
        return false;
    }

    nums.sort();
    let nums = nums.into_iter().rev().collect();

    let target_sum = total_array_sum / k;

    backtrack(&nums, 0, 0, 0, k, target_sum, 0, &mut HashMap::new())
}

pub fn can_partition_k_subsets_dp(nums: Vec<i32>, k: i32) -> bool {
    let mut total_array_sum = 0;
    let n = nums.len();

    for i in 0..n {
        total_array_sum += nums[i];
    }

    if total_array_sum % k != 0 {
        return false;
    }

    let target_sum = total_array_sum / k;

    let mut subset_sum = vec![-1; (1 << n) as usize];
    // Initially only one state is valid, i.e don't pick anything.
    subset_sum[0] = 0;

    for mask in 0..(1 << n) {
        if subset_sum[mask] == -1 {
            continue;
        }

        for i in 0..n {
            // If the number arr[i] was not picked earlier, and arr[i] + subsetSum[mask]
            // is not greater than the targetSum then add arr[i] to the subset
            // sum at subsetSum[mask] and store the result at subsetSum[mask | (1 << i)].

            if mask & (1 << i) == 0 && subset_sum[mask] + nums[i] <= target_sum {
                subset_sum[mask | (1 << i)] = (subset_sum[mask] + nums[i]) % target_sum;
            }
        }

        if subset_sum[(1 << n) - 1] == 0 {
            return true;
        }
    }

    subset_sum[(1 << n) - 1] == 0
}

// https://leetcode.com/problems/find-minimum-time-to-finish-all-jobs/description/
// https://leetcode.com/problems/find-minimum-time-to-finish-all-jobs/solutions/1009817/one-branch-cutting-trick-to-solve-three-leetcode-questions/
pub fn minimum_time_required_brute_tle(jobs: Vec<i32>, k: i32) -> i32 {
    fn dfs(jobs: &Vec<i32>, workers: &mut Vec<i32>, k: usize, i: usize) -> i32 {
        if i == jobs.len() {
            let mut ans = i32::MIN;
            for w in workers {
                ans = ans.max(*w);
            }
            return ans;
        }

        let mut ans = i32::MAX;
        for j in 0..k {
            workers[j] += jobs[i];
            ans = ans.min(dfs(jobs, workers, k, i + 1));
            workers[j] -= jobs[i];
        }
        ans
    }

    dfs(&jobs, &mut vec![0; k as usize], k as usize, 0)
}

pub fn minimum_time_required_dfs(mut jobs: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashSet;

    fn dfs(jobs: &Vec<i32>, workers: &mut Vec<i32>, k: usize, i: usize, ans: &mut i32) {
        if i == jobs.len() {
            *ans = (*ans).min(workers.iter().copied().max().unwrap());
            return;
        }

        let mut memo = HashSet::new();

        for j in 0..k {
            if memo.contains(&workers[j]) {
                continue;
            }

            if workers[j] + jobs[i] >= *ans {
                continue;
            }

            memo.insert(workers[j]);

            workers[j] += jobs[i];
            dfs(jobs, workers, k, i + 1, ans);
            workers[j] -= jobs[i];
        }
    }

    jobs.sort();
    jobs.reverse();

    let mut ans = i32::MAX;

    dfs(&jobs, &mut vec![0; k as usize], k as usize, 0, &mut ans);

    ans
}

// https://leetcode.com/problems/matchsticks-to-square/description/
pub fn makesquare(matchsticks: Vec<i32>) -> bool {
    todo!()
}
