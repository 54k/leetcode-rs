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
    for _ in 1..=*counter.get(&1).unwrap_or(&0) {
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

// https://leetcode.com/problems/longest-absolute-file-path/description/
pub fn length_longest_path(input: String) -> i32 {
    let split: Vec<&str> = input.split('\n').collect();

    let mut stack: Vec<(usize, &str)> = vec![];
    let mut ans = 0;

    for s in split {
        let mut lvl = 0;
        for ch in s.chars() {
            if ch == '\t' {
                lvl += 1;
            } else {
                break;
            }
        }
        let fname = &s[(lvl as usize)..];

        while stack.len() > 0 && stack[stack.len() - 1].0 >= lvl {
            stack.pop();
        }
        stack.push((lvl, fname.clone()));

        if fname.contains('.') {
            let v = stack
                .clone()
                .into_iter()
                .map(|(_, s)| s.clone().to_string())
                .collect::<Vec<String>>()
                .join("/");

            ans = ans.max(v.len() as i32);
        }
    }

    ans
}

// https://leetcode.com/problems/license-key-formatting/description/
pub fn license_key_formatting(s: String, k: i32) -> String {
    let mut ans = vec![];
    let s = s.chars().collect::<Vec<_>>();

    let mut cur_group_size = 0;

    for i in (0..s.len()).rev() {
        if s[i] == '-' {
            continue;
        }
        ans.push(s[i]);
        cur_group_size += 1;
        if cur_group_size == k {
            cur_group_size = 0;
            ans.push('-');
        }
    }

    let ans = ans.into_iter().rev().collect::<Vec<_>>();

    if ans.len() == 0 {
        return "".to_string();
    }

    let anss: String = if ans[0] == '-' {
        ans.into_iter().skip(1).collect()
    } else {
        ans.into_iter().collect()
    };

    anss.to_uppercase()
}
