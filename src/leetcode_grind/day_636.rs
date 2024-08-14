// https://leetcode.com/problems/kth-smallest-number-in-multiplication-table/description/
pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
    fn enough(x: i32, m: i32, n: i32, k: i32) -> bool {
        let mut count = 0;
        for i in 1..=m {
            count += n.min(x / i);
        }
        count >= k
    }
    let mut lo = 1;
    let mut hi = m * n;
    while lo < hi {
        let mi = lo + (hi - lo) / 2;
        if !enough(mi, m, n, k) {
            lo = mi + 1;
        } else {
            hi = mi;
        }
    }
    lo
}

// https://leetcode.com/problems/coin-path/description/
pub fn cheapest_jump(coins: Vec<i32>, max_jump: i32) -> Vec<i32> {
    let mut next = vec![-1; coins.len()];
    let mut dp = vec![0; coins.len()];
    let mut res = vec![];

    for i in (0..coins.len() - 1).rev() {
        let mut min_cost = i32::MAX;
        for j in (i + 1..=(i + max_jump as usize)) {
            if j >= coins.len() {
                break;
            }

            if coins[j as usize] >= 0 {
                let cost = coins[i] + dp[j];
                if cost < min_cost {
                    min_cost = cost;
                    next[i] = j as i32;
                }
            }
        }
        dp[i] = min_cost;
    }

    let mut i = 0;
    while i < coins.len() && next[i] > 0 {
        res.push(i as i32 + 1);
        i = next[i] as usize;
    }

    if i as usize == coins.len() - 1 && coins[i as usize] >= 0 {
        res.push(coins.len() as i32);
    } else {
        return vec![];
    }
    res
}
