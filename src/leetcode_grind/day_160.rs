// https://leetcode.com/problems/number-of-flowers-in-full-bloom/
pub fn full_bloom_flowers(mut flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
    // fn find_in_full_bloom(flowers: &Vec<Vec<i32>>, time: i32) -> i32 {
    //     let mut ans = 0;
    //     for (_, f) in flowers.iter().enumerate() {
    //         let (s, e) = (f[0], f[1]);
    //         if s <= time && e >= time {
    //             ans += 1;
    //         }
    //     }
    //     ans
    // }
    // flowers.sort();
    // let mut ans = vec![];
    // for p in people {
    //     ans.push(find_in_full_bloom(&flowers, p));
    // }
    // ans
    fn find_started_bloom(flowers: &Vec<Vec<i32>>, time: i32) -> i32 {
        let mut lo = 0;
        let mut hi = flowers.len() as i32 - 1;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if flowers[mid as usize][0] > time {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        hi
    }
    fn find_stopped_bloom(flowers: &Vec<Vec<i32>>, time: i32) -> i32 {
        let mut lo = 0i32;
        let mut hi = flowers.len() as i32 - 1;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if flowers[mid as usize][1] >= time {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        hi
    }
    flowers.sort();
    let mut fl = flowers.clone();
    fl.sort_by_key(|v| v[1]);
    let mut ans = vec![];
    for p in people {
        let i = find_started_bloom(&flowers, p);
        let j = find_stopped_bloom(&fl, p);
        ans.push(i - j);
    }
    ans
}

// https://leetcode.com/problems/profitable-schemes/description/
pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profits: Vec<i32>) -> i32 {
    const MOD: i64 = 1000000007;
    let mut dp = vec![vec![vec![0; 101]; 101]; 101];
    for count in 0..=n as usize {
        dp[group.len()][count][min_profit as usize] = 1;
    }

    for index in (0..group.len()).rev() {
        for count in 0..=n as usize {
            for profit in 0..=min_profit as usize {
                // ways to get a profitable scheme without this crime
                dp[index][count][profit] = dp[index + 1][count][profit];
                if count as i32 + group[index] <= n {
                    // adding ways to get profitable schemes, including this crime
                    dp[index][count][profit] = (dp[index][count][profit]
                        + dp[index + 1][count + group[index] as usize]
                            [(min_profit as usize).min(profit + profits[index] as usize)])
                        % MOD;
                }
            }
        }
    }
    dp[0][0][0] as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_449() {
        println!(
            "{:?}",
            full_bloom_flowers(
                vec![vec![1, 6], vec![3, 7], vec![9, 12], vec![4, 13]],
                vec![2, 3, 7, 11]
            )
        ); // 1,2,2,2
        println!(
            "{:?}",
            full_bloom_flowers(vec![vec![1, 10], vec![3, 3]], vec![3, 3, 2])
        ); // 2,2,1
    }

    #[test]
    fn test_450() {
        println!("{}", profitable_schemes(5, 3, vec![2, 3], vec![2, 2]));
    }
}
