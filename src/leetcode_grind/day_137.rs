pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
    fn top_down_approach(mut satisfaction: Vec<i32>) -> i32 {
        fn rec(satisfaction: &[i32], i: usize, time: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
            if i == satisfaction.len() {
                return 0;
            }
            if memo[i][time] != -1 {
                return memo[i][time];
            }
            memo[i][time] = (satisfaction[i] * time as i32
                + rec(satisfaction, i + 1, time + 1, memo))
            .max(rec(satisfaction, i + 1, time, memo));
            memo[i][time]
        }
        satisfaction.sort();
        let mut memo = vec![vec![-1; satisfaction.len() + 1]; satisfaction.len() + 1];
        rec(&satisfaction, 0, 1, &mut memo)
    }
    fn bottom_up_approach(mut satisfaction: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; satisfaction.len() + 2]; satisfaction.len() + 1];
        satisfaction.sort();
        for i in (0..satisfaction.len()).rev() {
            for j in 1..=satisfaction.len() {
                dp[i][j] = (satisfaction[i] * (j as i32) + dp[i + 1][j + 1]).max(dp[i + 1][j]);
            }
        }
        dp[0][1]
    }
    fn bottom_up_optimized_approach(mut satisfaction: Vec<i32>) -> i32 {
        let mut prev = vec![0; satisfaction.len() + 2];
        satisfaction.sort();
        for i in (0..satisfaction.len()).rev() {
            let mut dp = vec![0; satisfaction.len() + 2];
            for time in 1..=satisfaction.len() {
                dp[time] = (satisfaction[i] * time as i32 + prev[time + 1]).max(prev[time]);
            }
            prev = dp;
        }
        prev[1]
    }
    fn greedy_approach(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort();
        let mut max_satisfaction = 0;
        let mut suffix_sum = 0;
        for i in (0..satisfaction.len()).rev() {
            if suffix_sum + satisfaction[i] < 0 {
                break;
            }
            suffix_sum += satisfaction[i];
            max_satisfaction += suffix_sum;
        }
        max_satisfaction
    }
    greedy_approach(satisfaction)
}

// https://leetcode.com/problems/determine-if-two-events-have-conflict/
// https://leetcode.com/problems/determine-if-two-events-have-conflict/solutions/2735340/rust-one-liner/
pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
    // event1[0] <= event2[1] && event1[1] >= event2[0]
    fn to_num(event: Vec<String>) -> Vec<i32> {
        event
            .into_iter()
            .map(|x| {
                x.split(':')
                    .flat_map(|x| x.chars().map(|x| x.to_digit(10).unwrap() as i32))
                    .fold(0i32, |mut acc, v| {
                        acc = 10 * acc + v;
                        acc
                    })
            })
            .collect::<Vec<_>>()
    }
    let mut v = vec![to_num(event1), to_num(event2)];
    v.sort();
    v[0][1] >= v[1][0]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test382() {
        println!("{}", max_satisfaction(vec![-1, -8, 0, 5, -9])); // 14
        println!("{}", max_satisfaction(vec![4, 3, 2])); // 20
    }

    #[test]
    fn test383() {
        println!(
            "{}",
            have_conflict(
                vec!["01:15".to_string(), "02:00".to_string()],
                vec!["02:00".to_string(), "03:00".to_string()]
            )
        ); // true

        println!(
            "{}",
            have_conflict(
                vec!["01:00".to_string(), "02:00".to_string()],
                vec!["01:20".to_string(), "03:00".to_string()]
            )
        ); // true

        println!(
            "{}",
            have_conflict(
                vec!["10:00".to_string(), "11:00".to_string()],
                vec!["14:00".to_string(), "15:00".to_string()]
            )
        ); // false
    }
}
