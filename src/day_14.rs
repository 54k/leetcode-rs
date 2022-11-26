#[allow(dead_code)]
pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    let mut jobs = vec![];
    for i in 0..start_time.len() {
        jobs.push((start_time[i], end_time[i], profit[i]));
    }
    jobs.sort();

    let mut dp = vec![-1; jobs.len()];
    fn dfs(i: usize, jobs: &Vec<(i32, i32, i32)>, dp: &mut [i32]) -> i32 {
        if i == jobs.len() {
            return 0;
        }
        if dp[i] >= 0 {
            return dp[i];
        }

        let k = jobs.partition_point(|&job| job.0 < jobs[i].1);
        dp[i] = dfs(i + 1, jobs, dp).max(
            jobs[i].2 + dfs(k, jobs, dp)
        );
        dp[i]
    }

    dfs(0, &jobs, &mut dp)
}

#[cfg(test)]
mod test {
    use crate::day_14::*;

    #[test]
    fn test78() {
        println!("{}", job_scheduling(vec![1, 2, 3, 3],
                                      vec![3, 4, 5, 6],
                                      vec![50, 10, 40, 70])); // 120
    }
}