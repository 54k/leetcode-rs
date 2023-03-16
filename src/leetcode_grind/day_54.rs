#[allow(dead_code)]
pub fn unique_paths(n: i32, m: i32) -> i32 {
    let mut dp = vec![vec![1; m as usize]; n as usize];
    dp[0][0] = 1;
    for i in 1..n as usize {
        for j in 1..m as usize {
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
        }
    }

    dp[n as usize - 1][m as usize - 1]
}

#[allow(dead_code)]
pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    points.sort_by(|x1, x2| x1[1].cmp(&x2[1]));

    let mut ans = 1;
    let mut end = points[0][1];

    for i in 1..points.len() {
        let interval = &points[i];
        if interval[0] > end {
            end = end.max(interval[1]);
            ans += 1;
        }
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test146() {
        println!("{}", unique_paths(3, 7));
        println!("{}", unique_paths(2, 3));
    }

    #[test]
    fn test147() {
        println!(
            "{}",
            find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]])
        ); // 2

        println!(
            "{}",
            find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]])
        ); // 4

        println!(
            "{}",
            find_min_arrow_shots(vec![
                vec![9, 12],
                vec![1, 10],
                vec![4, 11],
                vec![8, 12],
                vec![3, 9],
                vec![6, 9],
                vec![6, 7]
            ])
        ); // 2
    }
}
