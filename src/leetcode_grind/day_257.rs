// https://leetcode.com/problems/maximum-running-time-of-n-computers/description/
pub fn max_run_time(n: i32, mut batteries: Vec<i32>) -> i64 {
    batteries.sort();
    let bn = batteries.len();

    let mut live = batteries
        .iter()
        .copied()
        .skip(bn - n as usize)
        .map(|x| x as i64)
        .collect::<Vec<_>>();

    let mut extra = batteries
        .iter()
        .copied()
        .take(bn - n as usize)
        .map(|x| x as i64)
        .sum::<i64>();

    for i in 0..n as usize - 1 {
        if extra < ((i + 1) as i64) * (live[i + 1] - live[i]) {
            return live[i] + extra / (i as i64 + 1);
        }

        extra -= (i as i64 + 1) * (live[i + 1] - live[i]);
    }

    live[n as usize - 1] + extra / n as i64
}
