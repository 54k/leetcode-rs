// https://leetcode.com/problems/smallest-sufficient-team/description/
pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
    use std::collections::HashMap;
    let n = people.len();
    let m = req_skills.len();

    let mut skill_id = HashMap::new();
    for i in 0..m {
        skill_id.insert(req_skills[i].clone(), i);
    }

    let mut skills_mask_of_person = vec![0; n];
    for i in 0..n {
        for skill in &people[i] {
            skills_mask_of_person[i] |= 1 << skill_id[skill];
        }
    }

    let mut dp: Vec<u64> = vec![(1 << n) - 1; 1 << m];
    dp[0] = 0;

    for skill_mask in 1..1 << m {
        for i in 0..n {
            let smaller_skills_mask = skill_mask & !skills_mask_of_person[i];
            if smaller_skills_mask == skill_mask {
                continue;
            }
            let people_mask = dp[smaller_skills_mask] | (1 << i);
            if (people_mask as u64).count_ones() < (dp[skill_mask] as u64).count_ones() {
                dp[skill_mask] = people_mask;
            }
        }
    }

    let answer_mask = dp[(1 << m) - 1];
    let mut ans = vec![];

    for i in 0..n {
        if ((answer_mask >> i) & 1) == 1 {
            ans.push(i as i32);
        }
    }

    ans
}

// https://leetcode.com/problems/the-number-of-good-subsets/description/
pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    const MOD: i64 = 1_000_000_007;
    const PRIMES: [i64; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

    let mut dp = vec![0i64; 1 << 10];
    dp[0] = 1;

    let mut counter: HashMap<i64, i64> = HashMap::new();
    for &n in &nums {
        *counter.entry(n as i64).or_insert(0) += 1;
    }

    for (a, count) in counter.clone() {
        if a == 1 {
            continue;
        }
        if a % 4 == 0 || a % 9 == 0 || a == 25 {
            continue;
        }

        let mut mask = 0;
        for (i, p) in PRIMES.iter().copied().enumerate() {
            if a % p == 0 {
                mask |= (1 << i);
            }
        }

        for i in 0..1 << 10 {
            if (i & mask) != 0 {
                continue;
            }

            dp[i | mask] += (count * dp[i]) % MOD;
        }
    }

    let mut ones = 1;
    for i in 1..=*counter.get(&1).unwrap_or(&0) {
        ones *= 2;
        ones %= MOD;
    }

    let mut dp_sum = -1;
    for d in dp {
        dp_sum += d;
        dp_sum %= MOD;
    }

    (((ones % MOD) * (dp_sum % MOD)) % MOD) as i32
}
