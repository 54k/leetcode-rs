// https://leetcode.com/problems/find-missing-observations/description/?envType=daily-question&envId=2024-09-05
pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
    let mut sum = 0;
    for &roll in &rolls {
        sum += roll;
    }

    let mut remaining_sum = mean * (n + rolls.len() as i32) - sum;

    if remaining_sum > 6 * n || remaining_sum < n {
        return vec![];
    }

    let mut distribute_mean = remaining_sum / n;
    let m = remaining_sum % n;

    let mut n_elements = vec![distribute_mean; n as usize];
    for i in 0..m as usize {
        n_elements[i] += 1;
    }
    n_elements
}
