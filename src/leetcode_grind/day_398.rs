// https://leetcode.com/problems/count-unique-characters-of-all-substrings-of-a-given-string/description/
pub fn unique_letter_string(s: String) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let n = s.len() as i64;
    const MOD: i64 = 1_000_000_007;
    let mut index = vec![vec![-1; 2]; 26];
    let mut res = 0;

    for i in 0..s.len() {
        let c = s[i] as usize - 'A' as usize;
        res = (res + (i as i64 - index[c][1]) * (index[c][1] - index[c][0]) % MOD) % MOD;
        index[c] = vec![index[c][1], i as i64];
    }

    for c in 0..26 {
        res = (res + (n - index[c][1]) * (index[c][1] - index[c][0]) % MOD) % MOD;
    }

    res as i32
}

// https://leetcode.com/problems/bulls-and-cows/description/
pub fn get_hint(secret: String, guess: String) -> String {
    let mut bulls = 0;
    let mut cows = 0;
    use std::collections::{HashMap, HashSet};

    let mut idx = HashMap::new();
    for (i, ch) in secret.chars().enumerate() {
        idx.entry(ch).or_insert(HashSet::new()).insert(i);
    }

    let mut skip = HashSet::new();

    for (i, ch) in guess.chars().enumerate() {
        if idx.contains_key(&ch) {
            if idx.get(&ch).unwrap().contains(&i) {
                bulls += 1;
                idx.get_mut(&ch).unwrap().remove(&i);
                if idx.get(&ch).unwrap().is_empty() {
                    idx.remove(&ch);
                }

                skip.insert(i);
            }
        }
    }

    for (i, ch) in guess.chars().enumerate() {
        if idx.contains_key(&ch) && !skip.contains(&i) {
            if !idx.get(&ch).unwrap().contains(&i) {
                cows += 1;
                let e = *idx.get(&ch).unwrap().iter().next().unwrap();
                idx.get_mut(&ch).unwrap().remove(&e);
                if idx.get(&ch).unwrap().is_empty() {
                    idx.remove(&ch);
                }
            }
        }
    }

    format!("{}A{}B", bulls, cows)
}

// https://leetcode.com/problems/additive-number/description/
pub fn is_additive_number(num: String) -> bool {
    fn backtrack(num: &Vec<char>, s: usize, prev: i64, sum: i64, count: i32) -> bool {
        if s == num.len() {
            return count > 2;
        }

        let mut cur = 0;

        for i in s..num.len() {
            let n = num[i] as i64 - '0' as i64;
            cur = cur * 10 + n;
            if count < 2 || cur == sum {
                if backtrack(num, i + 1, cur, cur + prev, count + 1) {
                    return true;
                }
            }
            if cur == 0 {
                break;
            }
        }

        false
    }

    let num = num.chars().collect::<Vec<_>>();
    backtrack(&num, 0, 0, 0, 0)
}

#[test]
fn test_is_additive_number() {
    let res = is_additive_number("1203".to_string());
    println!("{res}");

    let res = is_additive_number("111".to_string());
    println!("{res}");

    let res = is_additive_number("101".to_string());
    println!("{res}");

    let res = is_additive_number("112358".to_string());
    println!("{res}");

    let res = is_additive_number("199100199".to_string());
    println!("{res}");
}
