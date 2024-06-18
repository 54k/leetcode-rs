// https://leetcode.com/problems/most-profit-assigning-work/description
pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
    let mut job_profile = vec![];
    for i in 0..difficulty.len() {
        job_profile.push((difficulty[i], profit[i]));
    }
    let mut worker = worker;
    worker.sort();
    job_profile.sort_by_key(|x| x.0);

    let mut net_profit = 0;
    let mut max_profit = 0;
    let mut index = 0;

    for i in 0..worker.len() {
        while index < difficulty.len() && worker[i] >= job_profile[index].0 {
            max_profit = max_profit.max(job_profile[index].1);
            index += 1;
        }
        net_profit += max_profit;
    }
    net_profit
}
