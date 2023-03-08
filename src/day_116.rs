// https://leetcode.com/problems/koko-eating-bananas/
pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    fn can_eat_all(piles: &Vec<i32>, per_hour: i32, h: i32) -> bool {
        let mut hours_taken = 0;
        for i in 0..piles.len() {
            hours_taken += piles[i] / per_hour;
            if piles[i] % per_hour > 0 {
                hours_taken += 1;
            }
        }
        hours_taken <= h
    }
    let mut lo = 1;
    let mut hi = piles.iter().copied().max().unwrap();
    while lo < hi {
        let mid = (lo + hi) / 2;
        if can_eat_all(&piles, mid, h) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

// https://leetcode.com/problems/maximum-number-of-weeks-for-which-you-can-work/description/
// https://leetcode.com/problems/maximum-number-of-weeks-for-which-you-can-work/solutions/1375386/java-o-n-concise-explanation/
// As long as the weeks for the longest project <= all other projects' weeks + 1, we can finish all the projects.
// If the weeks for the longest project > all other projects' week + 1, we cannot finish the longest project.
// In this case, return (all other projects) * 2 + 1
pub fn number_of_weeks(milestones: Vec<i32>) -> i64 {
    let mut sum = 0;
    let mut max = 0;
    for m in milestones {
        sum += m as i64;
        max = max.max(m as i64);
    }
    if max <= sum - max {
        sum
    } else {
        2 * (sum - max) + 1
    }
}

// https://leetcode.com/problems/intervals-between-identical-elements/description/
// https://leetcode.com/problems/intervals-between-identical-elements/solutions/1647499/hash-and-formula/ dp approach
// https://leetcode.com/problems/intervals-between-identical-elements/solutions/1691107/rust-line-sweep-implementation/ line sweep
// https://leetcode.com/problems/intervals-between-identical-elements/solutions/1648143/C++-Simplest-O(N)-solution-till-now(I-promise-:))-No-separate-prefix-and-suffix-arrays/
pub fn get_distances(mut arr: Vec<i32>) -> Vec<i64> {
    fn dp_approach(mut arr: Vec<i32>) -> Vec<i64> {
        use std::collections::*;
        let mut m = HashMap::new();
        for i in 0..arr.len() {
            let len = m.len();
            m.entry(arr[i]).or_insert_with(|| len);
            arr[i] = m[&arr[i]] as i32;
        }
        let mut idx = vec![vec![]; m.len()];
        let mut sum = vec![0; m.len()];
        let mut last = vec![-1; m.len()];
        let mut res = vec![];

        for i in 0..arr.len() {
            idx[arr[i] as usize].push(i as i64);
            sum[arr[i] as usize] +=
                *idx[arr[i] as usize].last().unwrap() - *idx[arr[i] as usize].first().unwrap();
        }

        for i in 0..arr.len() {
            let p = last[arr[i] as usize];
            let v = arr[i] as usize;
            if p >= 0 {
                let p = p as usize;
                let diff = idx[v][p + 1] - idx[v][p];
                sum[v] += diff * p as i64 - diff * (idx[v].len() as i64 - 1 - (p as i64 + 1));
            }
            res.push(sum[v]);
            last[v] += 1;
        }
        res
    }
    fn line_sweep_approach(arr: Vec<i32>) -> Vec<i64> {
        // Line sweep idea.
        // For each vector of locations of same value,
        // maintain left and right sums and adjust the result
        // with the influence of each new item on the final result.
        use std::collections::*;
        let mut val_locs = HashMap::new();
        for i in 0..arr.len() {
            val_locs.entry(arr[i]).or_insert(vec![]).push(i);
        }
        let mut ans = vec![0; arr.len()];
        for locs in val_locs.values() {
            let mut left = 0i64;
            let mut right = locs.iter().sum::<usize>() as i64;
            let len = locs.len() as i64;

            for (pos, &idx) in locs.iter().enumerate() {
                let pos = pos as i64;
                let idx64 = idx as i64;
                right -= idx64;
                ans[idx] += right - idx64 * (len - pos - 1);
                ans[idx] -= left - idx64 * pos;
                left += idx64;
            }
        }
        ans
    }
    line_sweep_approach(arr)
}

// todo https://leetcode.com/problems/find-all-k-distant-indices-in-an-array/description/
// todo https://leetcode.com/problems/split-with-minimum-sum/description/
// todo https://leetcode.com/problems/task-scheduler/description/
// todo https://leetcode.com/problems/strong-password-checker/description/
// todo https://leetcode.com/problems/replace-non-coprime-numbers-in-array/description/

mod test {
    use super::*;

    #[test]
    fn test333() {
        println!("{}", min_eating_speed(vec![30, 11, 23, 4, 20], 5)); // 30
    }

    #[test]
    fn test334() {
        println!("{}", number_of_weeks(vec![1, 2, 3])); // 6
        println!("{}", number_of_weeks(vec![5, 2, 1])); // 7
    }

    #[test]
    fn test335() {
        println!("{:?}", get_distances(vec![2, 1, 3, 1, 2, 3, 3])); // [4,2,7,2,4,4,5]
        println!("{:?}", get_distances(vec![10, 5, 10, 10])); // [5,0,3,4]
    }
}
