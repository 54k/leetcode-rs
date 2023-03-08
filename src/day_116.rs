use std::collections::BinaryHeap;

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
}
