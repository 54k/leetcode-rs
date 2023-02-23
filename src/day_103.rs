use std::collections::HashMap;

// https://leetcode.com/problems/ipo/description/
// https://leetcode.com/problems/ipo/solutions/2959870/ipo/
pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    use std::collections::*;
    let mut heap = BinaryHeap::new();
    let mut sorted_projects = vec![];
    for i in 0..profits.len() {
        sorted_projects.push((capital[i], profits[i]));
    }
    sorted_projects.sort();

    let mut ptr = 0;
    for _ in 0..k {
        while ptr < sorted_projects.len() && sorted_projects[ptr].0 <= w {
            heap.push(sorted_projects[ptr].1);
            ptr += 1;
        }
        if heap.is_empty() {
            break;
        }
        w += heap.pop().unwrap();
    }
    w
}

// https://leetcode.com/problems/maximum-subsequence-score/
// https://leetcode.com/problems/maximum-subsequence-score/solutions/3092528/easiest-to-understand/
pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    use std::collections::*;

    let mut pairs = vec![];
    for i in 0..nums1.len() {
        pairs.push((nums2[i] as i64, nums1[i] as i64));
    }
    pairs.sort();
    pairs.reverse();

    let k = k as usize;
    let mut res = 0i64;
    let mut heap = BinaryHeap::new();
    let mut sum = 0;

    for (min_in_subsequence, subsequence_el) in pairs.iter() {
        heap.push(-subsequence_el);
        sum += subsequence_el;
        if heap.len() > k {
            sum -= -heap.pop().unwrap();
        }
        if heap.len() == k {
            res = res.max(sum * min_in_subsequence);
        }
    }
    res
}

// https://leetcode.com/problems/minimum-cost-to-hire-k-workers/
pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
    use std::collections::*;
    let k = k as usize;
    let mut ans = f64::MAX;
    let mut workers = vec![];
    for i in 0..quality.len() {
        workers.push((quality[i], wage[i] as f64 / quality[i] as f64));
    }
    workers.sort_by(|(_, r1), (_, r2)| r1.partial_cmp(r2).unwrap());

    let mut heap = BinaryHeap::new();
    let mut sum_quality = 0f64;

    for (quality, ratio) in workers {
        heap.push(quality);
        sum_quality += quality as f64;

        if heap.len() > k {
            sum_quality -= heap.pop().unwrap() as f64;
        }
        if heap.len() == k {
            let prod = sum_quality * ratio;
            if ans > prod {
                ans = prod;
            }
        }
    }
    ans
}

// https://leetcode.com/problems/maximize-the-confusion-of-an-exam/
pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
    fn linear(answer_key: String, k: i32) -> i32 {
        let answer_key = answer_key
            .chars()
            .map(|x| match x {
                'T' => 1,
                _ => 0,
            })
            .collect::<Vec<_>>();
        let mut ans = 0;
        let mut count_of_ones = 0;
        let mut count_of_zeros = 0;
        let mut left = 0;
        for i in 0..answer_key.len() {
            if answer_key[i] == 0 {
                count_of_zeros += 1;
            } else {
                count_of_ones += 1;
            }
            if count_of_ones <= k || count_of_zeros <= k {
                ans = ans.max(count_of_ones + count_of_zeros);
            }
            if count_of_ones > k && count_of_zeros > k {
                if answer_key[left] == 1 {
                    count_of_ones -= 1;
                } else {
                    count_of_zeros -= 1;
                }
                left += 1;
            }
        }
        ans
    }
    linear(answer_key, k)
}

// https://leetcode.com/problems/split-array-largest-sum/solutions/
// https://leetcode.com/problems/split-array-largest-sum/solutions/1899033/java-simple-and-easy-solution-beats-100/
pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
    fn can_split_into_chunks(nums: &[i32], mid: i32, k: i32) -> bool {
        let mut chunks = 0;
        let mut i = 0;
        while i < nums.len() {
            let mut val = 0;
            while i < nums.len() && nums[i] + val <= mid {
                val += nums[i];
                i += 1;
            }
            chunks += 1;
        }
        chunks <= k
    }
    let mut lo_sum = 0;
    let mut hi_sum = 0;
    for i in 0..nums.len() {
        lo_sum = lo_sum.max(nums[i]);
        hi_sum += nums[i];
    }
    while lo_sum < hi_sum {
        let mid = (lo_sum + hi_sum) / 2;
        if can_split_into_chunks(&nums, mid, k) {
            hi_sum = mid;
        } else {
            lo_sum = mid + 1;
        }
    }
    lo_sum
}

// https://leetcode.com/problems/magnetic-force-between-two-balls/
pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
    fn can_place_all_balls(position: &[i32], min_dist: i32, m: i32) -> bool {
        let mut balls_places = 1;
        let mut s = position[0];
        for i in 0..position.len() {
            if position[i] - s >= min_dist {
                balls_places += 1;
                s = position[i];
            }
        }
        balls_places >= m
    }
    position.sort();
    let mut ans = 0;
    let mut lo = 1;
    let mut hi = 1000000000;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if can_place_all_balls(&position, mid, m) {
            ans = mid;
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    ans
}

// https://leetcode.com/problems/minimized-maximum-of-products-distributed-to-any-store/description/
pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
    // Implement a function canDistribute(k), which returns true
    // if you can distribute all products such that any store will not be given more than k products,
    // and returns false if you cannot. Use this function to binary search for the smallest possible k.
    fn can_distribute(k: i32, quantities: &[i32], n: i32) -> bool {
        let mut distributed_stores = 0;
        for &quantity in quantities {
            if quantity < k {
                distributed_stores += 1;
            } else {
                distributed_stores += quantity / k + if quantity % k == 0 { 0 } else { 1 };
            }
        }
        distributed_stores <= n
    }
    let mut ans = 0;
    let mut lo = 1;
    let mut hi = 1000000000;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if can_distribute(mid, &quantities, n) {
            ans = mid;
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }
    ans
}

// https://leetcode.com/problems/minimum-number-of-days-to-eat-n-oranges/description/
// https://leetcode.com/problems/minimum-number-of-days-to-eat-n-oranges/solutions/794162/c-java-python-5-lines/
pub fn min_days(n: i32) -> i32 {
    use std::collections::*;
    let mut dp = HashMap::new();
    fn dfs(n: i32, dp: &mut HashMap<i32, i32>) -> i32 {
        if n <= 1 {
            return n;
        }
        if !dp.contains_key(&n) {
            let mid_days_to_eat = 1 + (n % 2 + dfs(n / 2, dp)).min(n % 3 + dfs(n / 3, dp));
            dp.insert(n, mid_days_to_eat);
        }
        dp.get(&n).copied().unwrap()
    }
    dfs(n, &mut dp)
}

// https://leetcode.com/problems/matrix-diagonal-sum/description/
pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
    let n = mat.len();
    let mut ans = 0;
    for i in 0..n {
        ans += mat[i][i] + mat[i][n - i - 1];
    }
    if n % 2 > 0 {
        ans -= mat[n / 2][n / 2];
    }
    ans
}

// https://leetcode.com/problems/count-integers-in-intervals/solutions/2039706/merge-intervals/
// https://leetcode.com/problems/count-integers-in-intervals/description/
struct CountIntervals {}

impl CountIntervals {
    fn new() -> Self {
        Self {}
    }

    fn add(&self, left: i32, right: i32) {}

    fn count(&self) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test276() {
        println!(
            "{}",
            find_maximized_capital(2, 0, vec![1, 2, 3], vec![1, 2, 3])
        ); // 4
    }

    #[test]
    fn test277() {
        println!("{}", max_score(vec![2, 1, 14, 12], vec![11, 7, 13, 6], 3)); // 168
        println!("{}", max_score(vec![1, 3, 3, 2], vec![2, 1, 3, 4], 3)); // 12
        println!(
            "{}",
            max_score(vec![4, 2, 3, 1, 1], vec![7, 5, 10, 9, 6], 1)
        ); // 30
    }

    #[test]
    fn test278() {
        println!(
            "{:?}",
            mincost_to_hire_workers(vec![10, 20, 5], vec![70, 50, 30], 2)
        ); // 150.00000
        println!(
            "{:?}",
            mincost_to_hire_workers(vec![3, 1, 10, 10, 1], vec![4, 8, 2, 2, 7], 3)
        ); // 30.66667
    }

    #[test]
    fn test279() {
        println!("{}", max_consecutive_answers("TTFF".to_string(), 2)); // 4
        println!("{}", max_consecutive_answers("TTTFFF".to_string(), 2)); // 5
        println!("{}", max_consecutive_answers("TFFT".to_string(), 1)); // 3
        println!("{}", max_consecutive_answers("FTFFTFTFTTFTTFTTFFTTFFTTTTTFTTTFTFFTTFFFFFTTTTFTTTTTTTTTFTTFFTTFTFFTTTFFFFFTTTFFTTTTFTFTFFTTFTTTTTTF".to_string(), 32));
        // 85
    }

    #[test]
    fn test280() {
        println!("{}", split_array(vec![7, 2, 5, 10, 8], 2)); // 18
        println!("{}", split_array(vec![1, 2, 3, 4, 5], 2)); // 9
    }

    #[test]
    fn test281() {
        println!("{}", max_distance(vec![1, 2, 3, 4, 7], 3)); // 3
        println!("{}", max_distance(vec![5, 4, 3, 2, 1, 1000000000], 2)); // 999999999
        println!("{}", max_distance(vec![79, 74, 57, 22], 4)); // 5
    }

    #[test]
    fn test282() {
        println!("{}", minimized_maximum(6, vec![11, 6])); // 3
        println!("{}", minimized_maximum(7, vec![15, 10, 10])); // 5
    }

    #[test]
    fn test283() {
        println!("{}", minimized_maximum(6, vec![11, 6])); // 3
        println!("{}", minimized_maximum(7, vec![15, 10, 10])); // 5
    }

    #[test]
    fn test284() {
        println!("{}", min_days(10)); // 4
        println!("{}", min_days(6)); // 3
    }

    #[test]
    fn test285() {
        println!(
            "{}",
            diagonal_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        ); // 25

        println!(
            "{}",
            diagonal_sum(vec![
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1]
            ])
        ); // 8
    }
}
