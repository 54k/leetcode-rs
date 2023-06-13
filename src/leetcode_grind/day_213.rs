// https://leetcode.com/problems/equal-row-and-column-pairs/description/
pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    let mut count = 0;
    let n = grid.len();
    let mut map = HashMap::new();
    for row in &grid {
        *map.entry(row.clone()).or_insert(0) += 1;
    }
    for c in 0..n {
        let mut col = vec![0; n];
        for r in 0..n {
            col[r] = grid[r][c];
        }
        count += *map.get(&col).unwrap_or(&0);
    }
    count
}

// https://leetcode.com/problems/stone-game-iv/description/
pub fn winner_square_game(n: i32) -> bool {
    pub fn using_top_down(n: i32) -> bool {
        use std::collections::HashMap;
        let mut cache = HashMap::new();
        cache.insert(0, false);
        fn dfs(rem: i32, cache: &mut HashMap<i32, bool>) -> bool {
            if !cache.contains_key(&rem) {
                let mut res = false;
                for k in 1..=((rem as f64).sqrt() as i32) {
                    if !dfs(rem - k * k, cache) {
                        cache.insert(rem, true);
                        return true;
                    }
                }
                cache.insert(rem, false);
            }
            cache[&rem]
        }
        dfs(n, &mut cache)
    }
    pub fn using_bottom_up(n: i32) -> bool {
        let mut dp = vec![false; n as usize + 1];
        for i in 0..=n as usize {
            let mut k = 1;
            while k * k <= i {
                if !dp[i - k * k] {
                    dp[i] = true;
                    break;
                }
                k += 1;
            }
        }
        dp[n as usize]
    }
    using_top_down(n)
}

// https://leetcode.com/problems/base-7/
pub fn convert_to_base7(mut num: i32) -> String {
    if num == 0 {
        return "0".to_owned();
    }
    let s = if num >= 0 { "" } else { "-" };
    let mut ans = String::new();
    while num != 0 {
        ans.push_str(&format!("{}", (num % 7).abs()));
        num /= 7;
    }
    ans.push_str(s);
    ans.chars().rev().collect()
}
