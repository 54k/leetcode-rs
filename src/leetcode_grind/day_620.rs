// https://leetcode.com/problems/count-number-of-teams/description/?envType=daily-question&envId=2024-07-29
pub fn num_teams_i(rating: Vec<i32>) -> i32 {
    let mut ans = 0;
    let n = rating.len();
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if rating[i] < rating[j] && rating[j] < rating[k] {
                    ans += 1;
                }

                if rating[i] > rating[j] && rating[j] > rating[k] {
                    ans += 1;
                }
            }
        }
    }
    ans
}

pub fn num_teams_iii(rating: Vec<i32>) -> i32 {
    let n = rating.len();
    let mut teams = 0;

    let mut increasing_teams = vec![vec![0; 4]; n];
    let mut decreasing_teams = vec![vec![0; 4]; n];

    for i in 0..n {
        increasing_teams[i][1] = 1;
        decreasing_teams[i][1] = 1;
    }

    for count in 2..=3 {
        for i in 0..n {
            for j in i + 1..n {
                if rating[j] > rating[i] {
                    increasing_teams[j][count] += increasing_teams[i][count - 1];
                }

                if rating[j] < rating[i] {
                    decreasing_teams[j][count] += decreasing_teams[i][count - 1];
                }
            }
        }
    }

    for i in 0..n {
        teams += increasing_teams[i][3] + decreasing_teams[i][3];
    }

    teams
}

pub fn num_teams_iv(rating: Vec<i32>) -> i32 {
    let mut max_rating = 0;
    for &r in &rating {
        max_rating = max_rating.max(r as usize);
    }

    let mut left_bit = vec![0; max_rating + 1];
    let mut right_bit = vec![0; max_rating + 1];

    fn update_bit(bit: &mut Vec<i32>, index: usize, value: i32) {
        let mut index = index as i32;
        while index < bit.len() as i32 {
            bit[index as usize] += value;
            index += index & -index;
        }
    }

    fn get_prefix_sum(bit: &Vec<i32>, index: usize) -> i32 {
        let mut sum = 0;
        let mut index = index as i32;
        while index > 0 {
            sum += bit[index as usize];
            index -= index & -index;
        }
        sum
    }

    for &r in &rating {
        update_bit(&mut right_bit, r as usize, 1);
    }

    let mut teams = 0;
    for &current_rating in &rating {
        update_bit(&mut right_bit, current_rating as usize, -1);
        let smaller_ratings_left = get_prefix_sum(&left_bit, current_rating as usize - 1);
        let smaller_ratings_right = get_prefix_sum(&right_bit, current_rating as usize - 1);

        let larger_ratings_left = get_prefix_sum(&left_bit, max_rating)
            - get_prefix_sum(&left_bit, current_rating as usize);
        let larget_ratings_right = get_prefix_sum(&right_bit, max_rating)
            - get_prefix_sum(&right_bit, current_rating as usize);

        teams += smaller_ratings_left * larget_ratings_right;
        teams += larger_ratings_left * smaller_ratings_right;

        update_bit(&mut left_bit, current_rating as usize, 1);
    }

    teams
}

// https://leetcode.com/problems/toss-strange-coins/description/?envType=weekly-question&envId=2024-07-29
pub fn probability_of_heads_i(prob: Vec<f64>, target: i32) -> f64 {
    let n = prob.len();
    let mut dp = vec![vec![0.0; target as usize + 1]; n + 1];
    dp[0][0] = 1.0;

    for i in 1..=n {
        dp[i][0] = dp[i - 1][0] * (1.0 - prob[i - 1]);
        let mut j = 1;
        while j <= target as usize && j <= i {
            dp[i][j] = dp[i - 1][j - 1] * prob[i - 1] + dp[i - 1][j] * (1.0 - prob[i - 1]);
            j += 1;
        }
    }

    dp[n][target as usize]
}

pub fn probability_of_heads_ii(prob: Vec<f64>, target: i32) -> f64 {
    let n = prob.len();
    let target = target as usize;
    let mut dp = vec![0.0; target + 1];
    dp[0] = 1.0;
    for i in 1..=n {
        for j in (1..=target).rev() {
            dp[j] = dp[j - 1] * prob[i - 1] + dp[j] * (1.0 - prob[i - 1]);
        }
        dp[0] = dp[0] * (1.0 - prob[i - 1]);
    }
    dp[target]
}
