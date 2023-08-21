// https://leetcode.com/problems/cracking-the-safe/description/
pub fn crack_safe(n: i32, k: i32) -> String {
    use std::collections::HashSet;

    fn crack_safe_after(
        pwd: &mut String,
        visited_combi: &mut HashSet<String>,
        target_num_visited: usize,
        n: usize,
        k: usize,
    ) -> bool {
        if visited_combi.len() == target_num_visited {
            return true;
        }
        let last_digits = pwd[pwd.len() - n + 1..].to_string();
        // println!("last digits {}", last_digits);

        let to = char::from_u32('0' as u32 + k as u32).unwrap();
        // println!("to: {}", to);

        for ch in '0'..to {
            let new_comb = format!("{}{}", last_digits, ch);
            // println!("new comb: {}", new_comb);

            if !visited_combi.contains(&new_comb) {
                visited_combi.insert(new_comb.clone());
                pwd.push(ch);
                if crack_safe_after(pwd, visited_combi, target_num_visited, n, k) {
                    return true;
                }
                visited_combi.remove(&new_comb);
                pwd.pop();
            }
        }

        false
    }

    let mut str_pwd = "0".repeat(n as usize);
    let mut visited_combi = HashSet::new();
    visited_combi.insert(str_pwd.clone());
    let target_num_visited = k.pow(n as u32) as usize;

    crack_safe_after(
        &mut str_pwd,
        &mut visited_combi,
        target_num_visited,
        n as usize,
        k as usize,
    );

    str_pwd
}
