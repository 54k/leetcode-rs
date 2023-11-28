//   https://leetcode.com/problems/number-of-ways-to-divide-a-long-corridor/description/
pub fn number_of_ways_1(corridor: String) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let corridor = corridor.chars().collect::<Vec<_>>();
    let n = corridor.len();
    let mut memo = vec![vec![-1; 3]; n];
    fn dp(index: usize, seats: usize, corridor: &Vec<char>, memo: &mut Vec<Vec<i32>>) -> i32 {
        if index == corridor.len() {
            return if seats == 2 { 1 } else { 0 };
        }

        if memo[index][seats] != -1 {
            return memo[index][seats];
        }

        let mut ans = 0;

        if seats == 2 {
            if corridor[index] == 'S' {
                ans = dp(index + 1, 1, corridor, memo);
            } else {
                ans = (ans + dp(index + 1, 0, corridor, memo)) % MOD;
                ans = (ans + dp(index + 1, seats, corridor, memo)) % MOD;
            }
        } else {
            if corridor[index] == 'S' {
                ans = dp(index + 1, seats + 1, corridor, memo);
            } else {
                ans = dp(index + 1, seats, corridor, memo);
            }
        }
        memo[index][seats] = ans;
        ans
    }

    dp(0, 0, &corridor, &mut memo)
}

pub fn number_of_ways_2(corridor: String) -> i32 {
    const MOD: i32 = 1_000_000_007;

    let n = corridor.len();
    let corridor = corridor.chars().collect::<Vec<_>>();

    let mut dp = vec![vec![0; 3]; n + 1];
    dp[n] = vec![0, 0, 1];

    for i in (0..n).rev() {
        if corridor[i] == 'S' {
            dp[i][0] = dp[i + 1][1];
            dp[i][1] = dp[i + 1][2];
            dp[i][2] = dp[i + 1][1];
        } else {
            dp[i][0] = dp[i + 1][0];
            dp[i][1] = dp[i + 1][1];
            dp[i][2] = (dp[i + 1][0] + dp[i + 1][2]) % MOD;
        }
    }

    dp[0][0]
}

pub fn number_of_ways_3(corridor: String) -> i32 {
    const MOD: i32 = 1_000_000_007;

    let n = corridor.len();
    let corridor = corridor.chars().collect::<Vec<_>>();

    let mut zero = 0;
    let mut one = 0;
    let mut two = 1;

    for i in (0..n).rev() {
        if corridor[i] == 'S' {
            zero = one;
            let tmp = one;
            one = two;
            two = tmp;
        } else {
            two = (zero + two) % MOD;
        }
    }

    zero
}

pub fn number_of_ways_4(corridor: String) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let corridor = corridor.chars().collect::<Vec<_>>();
    let mut count = 1i64;
    let mut seats = 0;
    let mut prev_pair_last = -1;
    for index in 0..corridor.len() {
        if corridor[index] == 'S' {
            seats += 1;
            if seats == 2 {
                prev_pair_last = index as i64;
                seats = 0;
            } else if seats == 1 && prev_pair_last != -1 {
                count *= index as i64 - prev_pair_last;
                count %= MOD;
            }
        }
    }

    if (seats == 1 || prev_pair_last == -1) {
        return 0;
    }
    count as i32
}

pub fn number_of_ways_5(corridor: String) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut ans = 1i64;

    let mut total = 0;
    let mut first = -1;
    let mut second = -1;

    let corridor = corridor.chars().collect::<Vec<_>>();
    for i in 0..corridor.len() {
        if corridor[i] == 'S' {
            total += 1;
            if first == -1 {
                first = i as i64;
                if second != -1 {
                    ans = (ans % MOD * (first - second) % MOD) % MOD;
                }
                second = -1;
            } else if second == -1 {
                second = i as i64;
                first = -1;
            }
        }
    }

    if total == 0 || total % 2 == 1 {
        return 0;
    }

    ans as i32
}
