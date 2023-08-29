// https://leetcode.com/problems/minimum-penalty-for-a-shop/description/
pub fn best_closing_time_i(customers: String) -> i32 {
    let customers = customers.chars().collect::<Vec<_>>();
    let mut cur_penalty = 0;
    for i in 0..customers.len() {
        if customers[i] == 'Y' {
            cur_penalty += 1;
        }
    }

    let mut min_penalty = cur_penalty;
    let mut earliest_hour = 0;

    for i in 0..customers.len() {
        let ch = customers[i];

        if ch == 'Y' {
            cur_penalty -= 1;
        } else {
            cur_penalty += 1;
        }

        if cur_penalty < min_penalty {
            earliest_hour = i + 1;
            min_penalty = cur_penalty;
        }
    }

    earliest_hour as i32
}

pub fn best_closing_time_ii(customers: String) -> i32 {
    let mut min_penalty = 0;
    let mut cur_penalty = 0;
    let mut earliest_hour = 0;

    for (i, ch) in customers.chars().into_iter().enumerate() {
        if ch == 'Y' {
            cur_penalty -= 1;
        } else {
            cur_penalty += 1;
        }

        if cur_penalty < min_penalty {
            earliest_hour = i + 1;
            min_penalty = cur_penalty;
        }
    }

    earliest_hour as i32
}

// https://leetcode.com/problems/find-permutation/
pub fn find_permutation(s: String) -> Vec<i32> {
    let mut res = vec![0; s.len() + 1];
    let mut stack = vec![];
    let mut j = 0;
    let s = s.chars().collect::<Vec<_>>();
    for i in 1..=s.len() {
        stack.push(i as i32);
        if s[i - 1] == 'I' {
            while !stack.is_empty() {
                res[j] = stack.pop().unwrap();
                j += 1;
            }
        }
    }
    stack.push(s.len() as i32 + 1);
    while !stack.is_empty() {
        res[j] = stack.pop().unwrap();
        j += 1;
    }
    res
}
