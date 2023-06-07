// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/
pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
    arr.sort();
    let diff = (arr[1] - arr[0]).abs();
    for i in 2..arr.len() {
        let d = (arr[i] - arr[i - 1]).abs();
        if diff != d {
            return false;
        }
    }
    true
}

// https://leetcode.com/problems/h-index-ii/description/
pub fn h_index_ii(citations: Vec<i32>) -> i32 {
    let n = citations.len() as i32;
    let mut lo = 0;
    let mut hi = citations.len() as i32 - 1;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        if citations[mid as usize] == n - mid {
            return citations[mid as usize];
        }
        if citations[mid as usize] < n - mid {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    n - lo
}

// https://leetcode.com/problems/paint-house-iii/description/
pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
    const MAX: i32 = 1000001;
    let mut dp = vec![vec![vec![MAX; target as usize + 1]; n as usize]; m as usize + 1];
    for j in 0..n as usize {
        if houses[0] as usize == j + 1 {
            dp[0][j][1] = 0;
        } else if houses[0] == 0 {
            dp[0][j][1] = cost[0][j];
        }
    }

    for i in 1..m as usize {
        for j in 0..n as usize {
            for k in 1..=target as usize {
                if houses[i] != 0 && j as i32 + 1 != houses[i] {
                    continue;
                }
                let mut curr_cost = MAX;
                for prev_color in 0..n as usize {
                    if j == prev_color {
                        curr_cost = curr_cost.min(dp[i - 1][prev_color][k]);
                    } else {
                        curr_cost = curr_cost.min(dp[i - 1][prev_color][k - 1]);
                    }
                }

                let cost_to_paint = if houses[i] != 0 { 0 } else { cost[i][j] };
                dp[i][j][k] = curr_cost + cost_to_paint;
            }
        }
    }

    let mut ans = MAX;
    for j in 0..n as usize {
        ans = ans.min(dp[m as usize - 1][j][target as usize]);
    }
    if ans == MAX {
        -1
    } else {
        ans
    }
}

// https://leetcode.com/problems/count-vowels-permutation/description/
pub fn count_vowel_permutation(n: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;

    let vows = vec![
        vec![1],          //a.0
        vec![0, 2],       //e.1
        vec![0, 1, 3, 4], //i.2
        vec![2, 4],       //o.3
        vec![0],          //u.4
    ];

    fn dp(n: i32, i: i32, next: usize, vows: &Vec<Vec<usize>>, cache: &mut Vec<Vec<i32>>) -> i32 {
        if i == n {
            return 1;
        }
        if cache[i as usize][next] != -1 {
            return cache[i as usize][next];
        }

        let mut cnt = 0;
        for &v in &vows[next] {
            cnt = (cnt % MOD + dp(n, i + 1, v, vows, cache) as i64 % MOD) % MOD;
        }
        cache[i as usize][next] = cnt as i32;
        cache[i as usize][next]
    }

    let mut cache = vec![vec![-1; 5]; n as usize];
    let mut ans = 0;
    for i in 0..vows.len() {
        ans = (ans % MOD + dp(n, 1, i, &vows, &mut cache) as i64 % MOD) % MOD;
    }
    ans as i32
}
