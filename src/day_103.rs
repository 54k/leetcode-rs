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
}
