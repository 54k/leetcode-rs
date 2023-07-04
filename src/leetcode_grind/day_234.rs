pub fn number_ways_top_down(hats: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    const MOD: i64 = 1000000007;

    let n = hats.len();
    let mut hats_to_people = HashMap::new();

    for i in 0..n {
        for &hat in &hats[i] {
            hats_to_people.entry(hat).or_insert(vec![]).push(i as i32);
        }
    }

    let done = (1 << n) - 1;
    let mut memo = vec![vec![-1; done]; 41];

    fn dp(
        hat: i32,
        mask: i32,
        done: i32,
        memo: &mut Vec<Vec<i32>>,
        hats_to_people: &HashMap<i32, Vec<i32>>,
    ) -> i32 {
        if mask == done {
            return 1;
        }

        if hat > 40 {
            return 0;
        }

        if memo[hat as usize][mask as usize] != -1 {
            return memo[hat as usize][mask as usize];
        }

        let mut ans = dp(hat + 1, mask, done, memo, hats_to_people);

        if hats_to_people.contains_key(&hat) {
            for &person in hats_to_people.get(&hat).unwrap() {
                if mask & (1 << person) == 0 {
                    ans = ((ans + dp(hat + 1, mask | (1 << person), done, memo, hats_to_people))
                        as i64
                        % MOD) as i32;
                }
            }
        }

        memo[hat as usize][mask as usize] = ans;
        ans
    }

    dp(1, 0, done as i32, &mut memo, &hats_to_people)
}

pub fn number_ways_bottom_up(hats: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    const MOD: i64 = 1000000007;

    let n = hats.len();
    let mut hats_to_people = HashMap::new();

    for i in 0..n {
        for &hat in &hats[i] {
            hats_to_people.entry(hat).or_insert(vec![]).push(i as i32);
        }
    }

    let done = (1 << n) - 1;
    let mut dp = vec![vec![0; done + 1]; 42];
    for i in 0..dp.len() {
        dp[i][done] = 1;
    }

    for hat in (0..41 as i32).rev() {
        for mask in (0..done + 1).rev() {
            let mut ans = dp[hat as usize + 1][mask];

            if hats_to_people.contains_key(&hat) {
                for &person in hats_to_people.get(&hat).unwrap() {
                    if mask & (1 << person) == 0 {
                        ans = ((ans + dp[hat as usize + 1][mask | (1 << person)]) as i64 % MOD)
                            as i32;
                    }
                }
            }

            dp[hat as usize][mask as usize] = ans;
        }
    }

    dp[1][0]
}
