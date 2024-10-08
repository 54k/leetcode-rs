// https://leetcode.com/problems/unique-length-3-palindromic-subsequences/description
pub fn count_palindromic_subsequence_1(s: String) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let mut counts_total = vec![0; 27];

    for &b in &s {
        counts_total[(b as i32 - 'a' as i32) as usize] += 1;
    }

    let mut count_running = vec![0; 27];

    use std::collections::HashSet;
    let mut set = HashSet::new();

    for i in 0..s.len() {
        for j in 0..=26 {
            let left = count_running[j];
            let right = counts_total[j]
                - left
                - if ((s[i] as i32 - 'a' as i32) as usize) == j {
                    1
                } else {
                    0
                };

            let side = char::from_u32(j as u32 + 'a' as u32).unwrap();
            let key = format!("{}{}{}", side, s[i], side);

            if left > 0 && right > 0 && !set.contains(&key) {
                set.insert(key);
            }
        }

        count_running[(s[i] as i32 - 'a' as i32) as usize] += 1;
    }
    // println!("{:?}", set);
    set.len() as i32
}

fn gen() -> Vec<String> {
    let mut res = vec![];

    fn go(pat: &Vec<char>, curr: &mut Vec<char>, res: &mut Vec<String>, used: &mut Vec<bool>) {
        if curr.len() == pat.len() {
            res.push(curr.clone().into_iter().collect());
            return;
        }

        for i in 0..pat.len() {
            if used[i] {
                continue;
            }
            used[i] = true;
            curr.push(pat[i]);
            go(pat, curr, res, used);
            curr.pop();
            used[i] = false;
        }
    }

    for pat in ["check", "data", "format"] {
        let pat = pat.to_string().chars().collect::<Vec<_>>();
        go(&pat, &mut vec![], &mut res, &mut vec![false; pat.len()]);
    }

    for i in 0..res.len() - 2 {
        for j in i + 1..res.len() - 1 {
            for k in 0..res.len() {
                let s = format!("{}_{}_{} = check_data_format", res[i], res[j], res[k]);
                println!("{s}");
            }
        }
    }

    res
}

#[test]
fn test_gen() {
    gen();
}
