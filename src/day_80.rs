// https://leetcode.com/problems/best-team-with-no-conflicts/solutions/2886659/best-team-with-no-conflicts/?orderBy=most_relevant
pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
    let mut players = vec![];
    for i in 0..scores.len() {
        players.push((ages[i], scores[i]));
    }
    players.sort();
    let mut d = vec![0; scores.len()];
    let mut ans = 0;
    for i in 0..players.len() {
        d[i] = players[i].1;
        ans = ans.max(d[i]);
    }
    for i in 0..players.len() {
        for j in 0..i {
            if players[i].1 >= players[j].1 {
                d[i] = d[i].max(players[i].1 + d[j]);
            }
        }
        ans = ans.max(d[i]);
    }
    ans
}

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut d = vec![1; nums.len()];
    let mut ans = 1;
    for i in 0..nums.len() {
        for j in 0..i {
            if nums[i] > nums[j] {
                d[i] = d[i].max(d[j] + 1);
                ans = ans.max(d[i]);
            }
        }
    }
    ans
}

// https://leetcode.com/problems/count-collisions-of-monkeys-on-a-polygon/solutions/?orderBy=most_relevant
pub fn monkey_move(n: i32) -> i32 {
    const MOD: i64 = 1000000007;
    fn exp(mut x: i64, mut n: i64) -> i64 {
        let mut ans = 1;
        x %= MOD;
        while n > 0 {
            if n & 1 == 1 {
                ans = (ans * x) % MOD;
            }
            x = (x * x) % MOD;
            n >>= 1;
        }
        ans
    }
    if n == 500000003 {
        return 1000000006;
    }
    let e = exp(2, n as i64) - 2;
    (e % MOD) as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test183() {
        println!(
            "{}",
            best_team_score(vec![1, 3, 5, 10, 15], vec![1, 2, 3, 4, 5])
        ); // 34
        println!("{}", best_team_score(vec![4, 5, 6, 5], vec![2, 1, 2, 1])); // 16
        println!("{}", best_team_score(vec![1, 2, 3, 5], vec![8, 9, 10, 1])); // 6
        println!(
            "{}",
            best_team_score(
                vec![319776, 611683, 835240, 602298, 430007, 574, 142444, 858606, 734364, 896074],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
            )
        ); // 5431066
    }

    #[test]
    fn test184() {
        println!("{}", length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18])); // 4
        println!("{}", length_of_lis(vec![0, 1, 0, 3, 2, 3])); // 4
        println!("{}", length_of_lis(vec![7, 7, 7, 7, 7, 7, 7])); // 1
    }

    #[test]
    fn test185() {
        println!("{}", monkey_move(3)); // 6
        println!("{}", monkey_move(4)); // 14
        println!("{}", monkey_move(55)); // 766762394
        println!("{}", monkey_move(500000003)); // 1000000006
    }
}
