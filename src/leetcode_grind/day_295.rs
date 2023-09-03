// https://leetcode.com/problems/unique-paths/
pub fn unique_paths(m: i32, n: i32) -> i32 {
    let mut dp = vec![0; 10000];
    dp[0] = 1;

    for i in 0..m as usize {
        let mut next = vec![0; 10000];

        for j in 0..n as usize {
            next[j] += dp[j];
            if j > 0 {
                next[j] += next[j - 1];
            }
        }
        dp = next;
    }

    dp[n as usize - 1]
}

// https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses/
pub fn reverse_parentheses(s: String) -> String {
    let mut stack: Vec<String> = vec!["".to_string()];
    for ch in s.chars() {
        match ch {
            ')' => {
                let top = stack.pop().unwrap();
                let top = top.chars().into_iter().rev().collect::<String>();
                stack.last_mut().unwrap().extend(top.chars());
            }
            '(' => {
                stack.push(String::new());
            }
            _ => {
                stack.last_mut().unwrap().push(ch);
            }
        }
    }
    stack.pop().unwrap()
}

// https://leetcode.com/problems/score-of-parentheses/
pub fn score_of_parentheses_i(s: String) -> i32 {
    let mut score = 0;
    let mut bal = 0;
    let s = s.chars().collect::<Vec<_>>();
    for i in 0..s.len() {
        let ch = s[i];
        if ch == '(' {
            bal += 1;
        }
        if ch == ')' {
            bal -= 1;
            if s[i - 1] == '(' {
                score += 1 << bal;
            }
        }
    }
    score
}

pub fn score_of_parentheses_ii(s: String) -> i32 {
    fn divide_and_conquer(s: &Vec<char>, mut i: usize, j: usize) -> i32 {
        let mut ans = 0;
        let mut bal = 0;
        for k in i..j {
            bal += if s[k] == '(' { 1 } else { -1 };

            if bal == 0 {
                if k - i == 1 {
                    ans += 1;
                } else {
                    ans += 2 * divide_and_conquer(s, i + 1, k);
                }
                i = k + 1;
            }
        }
        ans
    }
    let s = s.chars().collect::<Vec<_>>();
    divide_and_conquer(&s, 0, s.len())
}

pub fn score_of_parentheses_iii(s: String) -> i32 {
    let mut stack = vec![0];

    for ch in s.chars() {
        if ch == '(' {
            stack.push(0);
        } else {
            let v = stack.pop().unwrap();
            let w = stack.pop().unwrap();
            stack.push(w + (2 * v).max(1));
        }
    }

    stack.pop().unwrap()
}
