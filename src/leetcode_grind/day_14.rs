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
        dp[i] = dfs(i + 1, jobs, dp).max(jobs[i].2 + dfs(k, jobs, dp));
        dp[i]
    }

    dfs(0, &jobs, &mut dp)
}

#[allow(dead_code)]
pub fn climb_stairs(n: i32) -> i32 {
    fn climb_stairs_rec(n: i32, dp: &mut [i32]) -> i32 {
        if n < 0 {
            return 0;
        }
        if n == 0 {
            return 1;
        }
        let n = n as usize;
        if dp[n] >= 0 {
            return dp[n];
        }
        dp[n] = climb_stairs_rec(n as i32 - 1, dp) + climb_stairs_rec(n as i32 - 2, dp);
        dp[n]
    }

    fn climb_stairs_fib(n: i32, dp: &mut [i32]) -> i32 {
        if n < 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }

        for i in 2..=n as usize {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n as usize]
    }

    let mut fib = vec![0; n as usize + 1];
    fib[0] = 1;
    fib[1] = 1;

    climb_stairs_fib(n, &mut vec![-1; n as usize + 1])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test78() {
        println!(
            "{}",
            job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70])
        ); // 120
    }

    #[test]
    fn test79() {
        println!("{}", climb_stairs(3)); //3
    }
}
