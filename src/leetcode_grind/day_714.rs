// https://leetcode.com/problems/minimum-total-distance-traveled/description/?envType=daily-question&envId=2024-10-31
pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
    robot.sort();
    factory.sort();

    let mut factory_positions = vec![];
    for f in factory {
        for _ in 0..f[1] {
            factory_positions.push(f[0]);
        }
    }

    let mut robot_count = robot.len();
    let mut factory_count = factory_positions.len();

    let mut dp = vec![vec![0; factory_count + 1]; robot_count + 1];
    for i in 0..robot_count {
        dp[i][factory_count] = 1e12 as i64;
    }

    for i in (0..robot_count).rev() {
        for j in (0..factory_count).rev() {
            let assign = (robot[i] - factory_positions[j]).abs() as i64 + dp[i + 1][j + 1];
            let skip = dp[i][j + 1];
            dp[i][j] = assign.min(skip);
        }
    }

    dp[0][0]
}
