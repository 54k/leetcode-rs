// https://leetcode.com/problems/minimum-time-to-complete-trips/editorial/
pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
    fn can_make_total_trips(time: &Vec<i32>, time_given: i64, total_trips: i32) -> bool {
        let mut trips = 0;
        for &t in time {
            trips += time_given / (t as i64);
        }
        trips >= total_trips as i64
    }

    let mut lo = 1;
    let mut hi = time.iter().copied().max().unwrap() as i64 * total_trips as i64;
    while lo < hi {
        let time_given = (lo + hi) / 2;
        if can_make_total_trips(&time, time_given, total_trips) {
            hi = time_given;
        } else {
            lo = time_given + 1;
        }
    }
    lo
}

//https://leetcode.com/problems/count-ways-to-group-overlapping-ranges/description/
// https://leetcode.com/problems/count-ways-to-group-overlapping-ranges/solutions/3256086/c-merging-counting-efficient-approach/?orderBy=most_relevant
pub fn count_ways(mut ranges: Vec<Vec<i32>>) -> i32 {
    ranges.sort();
    const MOD: i32 = 1000000007;
    // We need to find total number of non-overlapping range i.e. total and possible ways will be 2^(total).
    let mut total = 1;
    let mut end = ranges[0][1];
    for i in 1..ranges.len() {
        if ranges[i][0] > end {
            total += 1;
        }
        end = end.max(ranges[i][1]);
    }

    let mut ans = 1;
    while total > 0 {
        ans = (ans * 2) % MOD;
        total -= 1;
    }
    ans
}

// https://leetcode.com/problems/subarray-sum-equals-k/description/
// https://leetcode.com/problems/subarray-sum-equals-k/editorial/
pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;
    let mut sum = 0;
    let mut ans = 0;
    let mut num_of_occurences = vec![(0, 1)].into_iter().collect::<HashMap<i32, i32>>();
    for i in 0..nums.len() {
        sum += nums[i];
        if num_of_occurences.contains_key(&(sum - k)) {
            ans += *num_of_occurences.get(&(sum - k)).unwrap();
        }
        *num_of_occurences.entry(sum).or_insert(0) += 1;
    }
    ans
}

// https://leetcode.com/problems/continuous-subarray-sum/description/
// https://leetcode.com/problems/continuous-subarray-sum/editorial/
pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    // tle
    fn brute(nums: Vec<i32>, k: i32) -> bool {
        let mut prefix = vec![0i64; nums.len() + 1];
        for i in 1..=nums.len() {
            prefix[i] = nums[i - 1] as i64 + prefix[i - 1];
        }
        for i in 0..nums.len() {
            for j in i + 1..=nums.len() {
                let subarray_sum = prefix[j] - prefix[i];
                if j - i >= 2 && subarray_sum % k as i64 == 0 {
                    return true;
                }
            }
        }
        false
    }
    fn optimized(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashMap;
        let k = k as i64;
        let mut running_sum = 0i64;
        let mut sums_with_idx = vec![(0, -1)].into_iter().collect::<HashMap<i64, i32>>();
        for i in 0..nums.len() {
            running_sum += nums[i] as i64;
            if i as i32 - *sums_with_idx.get(&(running_sum % k)).unwrap_or(&(i as i32)) >= 2 {
                return true;
            }
            sums_with_idx.entry(running_sum % k).or_insert(i as i32);
        }
        false
    }
    optimized(nums, k)
}

// https://leetcode.com/problems/array-partition/description/
pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
    todo!()
}

// todo https://leetcode.com/problems/replace-non-coprime-numbers-in-array/description/

mod test {
    use super::*;

    #[test]
    fn test327() {
        println!("{}", minimum_time(vec![5, 10, 10], 9)); // 25
        println!("{}", minimum_time(vec![1, 2, 3], 5)); // 3
        println!("{}", minimum_time(vec![2], 1)); // 2
    }

    #[test]
    fn test328() {
        println!("{}", count_ways(vec![vec![6, 10], vec![5, 15]])); // 2
        println!(
            "{}",
            count_ways(vec![vec![1, 3], vec![10, 20], vec![2, 5], vec![4, 8]])
        ); // 4
    }

    #[test]
    fn test329() {
        println!("{}", subarray_sum(vec![1, 1, 1], 2)); // 2
        println!("{}", subarray_sum(vec![1, 2, 3], 3)); // 2
        println!("{}", subarray_sum(vec![1, -1, 0], 0)); // 3
    }

    #[test]
    fn test330() {
        println!("{}", check_subarray_sum(vec![23, 2, 4, 6, 6], 7)); // true
        println!("{}", check_subarray_sum(vec![23, 2, 4, 6, 7], 6)); // true
        println!("{}", check_subarray_sum(vec![23, 2, 6, 4, 7], 6)); // true
        println!("{}", check_subarray_sum(vec![23, 2, 6, 4, 7], 13)); // false
        println!("{}", check_subarray_sum(vec![23, 6, 9], 6)); // false
        println!("{}", check_subarray_sum(vec![0, 0], 1)); // true
    }
}
