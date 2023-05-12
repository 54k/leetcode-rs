// https://leetcode.com/problems/find-k-th-smallest-pair-distance/description/
pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();
    let mut lo = 0;
    let mut hi = nums[nums.len() - 1] - nums[0];

    while lo < hi {
        let mid = (lo + hi) / 2;
        let mut count = 0;
        let mut left = 0;

        for right in 0..nums.len() {
            while nums[right] - nums[left] > mid {
                left += 1;
            }
            count += right - left;
        }

        if count >= k as usize {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

// https://leetcode.com/problems/find-k-th-smallest-pair-distance/description/
pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
    let mut sum_power = 0;
    for &p in &batteries {
        sum_power += p as i64;
    }

    let mut lo = 1;
    let mut hi = sum_power / n as i64;

    while lo < hi {
        let target = hi - (hi - lo) / 2;
        let mut extra = 0;

        for &p in &batteries {
            let p = p as i64;
            extra += p.min(target);
        }

        if extra >= n as i64 * target {
            lo = target;
        } else {
            hi = target - 1;
        }
    }

    lo
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test500() {
        println!("{:?}", smallest_distance_pair(vec![1, 3, 1], 1));
    }
}
