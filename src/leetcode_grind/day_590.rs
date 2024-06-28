// https://leetcode.com/problems/maximum-total-importance-of-roads/description/?envType=daily-question&envId=2024-06-28
pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
    let mut degree = vec![0; n as usize];
    for edge in roads {
        degree[edge[0] as usize] += 1;
        degree[edge[1] as usize] += 1;
    }
    degree.sort();

    let mut value = 1;
    let mut total_importance = 0;
    for d in degree {
        total_importance += (value * d);
        value += 1;
    }

    total_importance
}
