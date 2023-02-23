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
}
