use std::cmp::{Ordering, Reverse};

fn problem2(n: i32, m: i32) -> i32 {
    fn rec(n: i32, m: i32, i: usize, ans: &mut i32) {
        if n == i as i32 {
            *ans += 1;
            return;
        }
        for _ in 1..=m {
            rec(n, m, i + 1, ans);
        }
    }
    let mut ans = 0;
    rec(n, m, 0, &mut ans);
    ans
}

fn problem3(n: i32, at: usize) -> String {
    fn rec(n: i32, i: i32, bal: i32, buf: &mut String, acc: &mut Vec<String>) {
        if n * 2 == i {
            if bal == 0 {
                acc.push(buf.clone());
            }
            return;
        }
        buf.push('(');
        rec(n, i + 1, bal + 1, buf, acc);
        buf.pop();
        if bal == 0 {
            return;
        }
        buf.push(')');
        rec(n, i + 1, bal - 1, buf, acc);
        buf.pop();
    }
    let mut acc = vec![];
    rec(n, 0, 0, &mut String::new(), &mut acc);
    acc[at].clone()
}

fn problem4() {
    fn rec(
        n: usize,
        idx: usize,
        len: usize,
        used: &mut Vec<bool>,
        p: &mut Vec<usize>,
        a: &Vec<Vec<usize>>,
        ans_v: &mut Vec<usize>,
    ) {
        if n == idx {
            let l = len + a[p[idx - 1]][0];
            if ans_v[0] > l {
                let mut v = vec![l];
                v.extend(p.clone());
                *ans_v = v;
            }
            return;
        }
        for i in 1..n {
            if used[i] {
                continue;
            }
            p[idx] = i;
            used[i] = true;
            rec(n, idx + 1, len + a[p[idx - 1]][i], used, p, a, ans_v);
            used[i] = false;
        }
    }

    let mut lines = include_str!("salesman.in").lines();
    let n = lines.next().unwrap().parse::<usize>().unwrap();
    let mut p = vec![0; n];
    let mut a = vec![vec![0; n]; n];
    let mut used = vec![false; n];
    for (i, line) in lines.enumerate() {
        for (j, v) in line
            .split(' ')
            .map(|num| num.parse::<usize>().unwrap())
            .enumerate()
        {
            a[i][j] = v;
        }
    }
    let mut ans_v = vec![usize::MAX];
    rec(n, 1, 0, &mut used, &mut p, &a, &mut ans_v);
    println!("{}", ans_v[0]);
    ans_v.into_iter().skip(1).for_each(|f| print!("{} ", f));
    println!();
}

fn problem5() {
    fn valid(str: &str) -> bool {
        let mut s = vec![];
        for ch in str.chars() {
            match ch {
                '(' | '[' => {
                    s.push(ch);
                }
                ')' if !s.is_empty() && s[s.len() - 1] == '(' => {
                    s.pop();
                }
                ']' if !s.is_empty() && s[s.len() - 1] == '[' => {
                    s.pop();
                }
                _ => return false,
            };
        }
        s.is_empty()
    }
    let mut lines = include_str!("brackets.in").lines();
    println!("{:?}", lines.map(|f| valid(f) as i32).collect::<Vec<_>>());
}

fn problem6(n: i32) {
    fn rec(n: i32, len: i32, buf: &mut String, ans: &mut Vec<String>) {
        //noinspection ALL
        fn valid(str: &str) -> bool {
            let mut s = vec![];
            for ch in str.chars() {
                match ch {
                    '(' | '[' => {
                        s.push(ch);
                    }
                    ')' if !s.is_empty() && s[s.len() - 1] == '(' => {
                        s.pop();
                    }
                    ']' if !s.is_empty() && s[s.len() - 1] == '[' => {
                        s.pop();
                    }
                    _ => return false,
                };
            }
            s.is_empty()
        }
        if n * 2 == len {
            if valid(buf) {
                ans.push(buf.clone());
            }
            return;
        }

        for ch in ['(', ')', '[', ']'] {
            buf.push(ch);
            rec(n, len + 1, buf, ans);
            buf.pop();
        }
    }

    let mut ans = vec![];
    rec(n, 0, &mut String::new(), &mut ans);
    println!("{:?}", ans.len());
    println!("{:?}", &ans[19]);
}

fn problem7(n: i32) {
    // catalan numbers
    fn fff(n: i32) -> Vec<String> {
        let mut res = vec![];
        if n == 0 {
            res.push("".to_string());
            return res;
        }
        for c in 0..n {
            for first in fff(c) {
                for last in fff(n - 1 - c) {
                    res.push(format!("({}){}", first, last));
                    res.push(format!("[{}]{}", first, last));
                }
            }
        }
        res
    }
    let mut ans = fff(n);
    ans.sort();
    println!("{}", ans.len());
    println!("{:?}", &ans[8232]);
}

fn problem3_4_and_5() {
    let mut s = include_str!("cont.in").lines();
    let n_w = s
        .next()
        .unwrap()
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let n = n_w[0];
    let mut total = n_w[1];

    let mut w = vec![(0, 0); n];

    for (i, line) in s.enumerate() {
        let w_c = line
            .split(' ')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        w[i] = (w_c[0], w_c[1]);
    }

    w.sort_by(|a, b| {
        if a.1 == 0 {
            return Ordering::Less;
        }
        if b.1 == 0 {
            return Ordering::Greater;
        }
        (a.0 as f64 / a.1 as f64).total_cmp(&(b.0 as f64 / b.1 as f64))
    });
    let mut ans = 0;

    for i in 0..n {
        if w[i].0 < total {
            total -= w[i].0;
            ans += w[i].1;
        } else {
            let p = (w[i].1 / w[i].0) * total;
            ans += p;
            total = 0;
        }
    }
    println!("{}", ans);
}

fn problem3_6() {
    let mut lines = include_str!("request2.in").lines();

    let n = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|f| f.parse::<usize>().unwrap())
        .take(1)
        .last()
        .unwrap();

    let mut meetings = vec![vec![0; 2]; n];
    for (i, line) in lines.enumerate() {
        let a_b = line
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        meetings[i][0] = a_b[0];
        meetings[i][1] = a_b[1];
    }
    meetings.sort();

    use std::collections::BinaryHeap;
    let mut rooms = BinaryHeap::new();
    rooms.push(Reverse(meetings[0][1]));
    for m in meetings.into_iter().skip(1) {
        if m[0] >= rooms.peek().unwrap().0 {
            rooms.pop();
        }
        rooms.push(Reverse(m[1]));
    }
    println!("{:?}", rooms.len());

    // let mut rooms = vec![];
    // rooms.push(meetings[0][1]);
    // 'outer: for m in meetings.into_iter().skip(1) {
    //     for time_end in &mut rooms {
    //         if m[0] >= *time_end {
    //             *time_end = m[1];
    //             continue 'outer;
    //         }
    //     }
    //     rooms.push(m[1]);
    // }
    // println!("{}", rooms.len());
}

fn problem_3_7() {
    use std::collections::BinaryHeap;
    let mut lines = include_str!("contest.in").lines();
    let mut t = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .skip(1)
        .collect::<Vec<_>>()[0];

    let mut tasks = BinaryHeap::new();
    lines
        .next()
        .unwrap()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .for_each(|task| tasks.push(Reverse(task)));

    let mut prev_penalty = 0;
    let mut total_penalty = 0;
    let mut count = 0;
    while let Some(Reverse(task)) = tasks.pop() {
        if task < t {
            t -= task;
            count += 1;
            let penalty = prev_penalty + task;
            prev_penalty = penalty;
            total_penalty += prev_penalty;
        }
    }
    println!("{} {}", count, total_penalty);
}

fn problem_3_8() {
    use std::collections::HashSet;
    let lines = include_str!("ice2.in").lines();
    let v = lines.skip(1).collect::<Vec<_>>();
    let mut s = HashSet::new();
    let mut ans = 1;
    for str in v {
        if s.contains(str) {
            ans += 1;
            s.clear();
        }
        s.insert(str);
    }
    println!("{}", ans);
}

fn problem_dominos(n: usize) {
    let mut fib = vec![0u128; n + 1];
    fib[1] = 1;
    fib[2] = 1;
    fib[3] = 2;
    for i in 4..=n {
        fib[i] = fib[i - 1] + fib[i - 3];
    }
    println!("{}", fib[n]);
}

fn problem_bug(a: &mut [&mut [i32]]) {
    let n = a.len();
    let m = a[0].len();
    let mut dp = vec![vec![0; m]; n];
    let mut path = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            dp[i][j] = a[i][j];
            if i > 0 && dp[i][j] < dp[i - 1][j] + a[i][j] {
                dp[i][j] = dp[i - 1][j] + a[i][j];
            }
            if j > 0 && dp[i][j] < dp[i][j - 1] + a[i][j] {
                dp[i][j] = dp[i][j - 1] + a[i][j];
                path[i][j] = 1;
            }
        }
    }

    fn path_rec(path: &[Vec<i32>], i: usize, j: usize) {
        if i == 0 && j == 0 {
            return;
        }
        if path[i][j] == 1 {
            path_rec(path, i, j - 1);
            print!("R");
        } else {
            path_rec(path, i - 1, j);
            print!("D");
        }
    }

    path_rec(&path, n - 1, m - 1);
    println!();
    dp.into_iter().for_each(|d| {
        println!("{:?}", d);
    })
}

fn problem_knapsack(n: usize, w: usize, weights: &[i32], values: &[i32]) {
    fn print_path(dp: &Vec<Vec<i32>>, weights: &[i32], i: usize, j: usize) {
        if i == 0 {
            return;
        }
        if i > 0 && dp[i][j] == dp[i - 1][j] {
            print_path(dp, weights, i - 1, j);
        } else {
            print_path(dp, weights, i - 1, j - weights[i] as usize);
            print!("{} ", i);
        }
    }
    let mut dp = vec![vec![0; w + 1]; n + 1];
    for i in 1..=n {
        for j in 0..=w {
            dp[i][j] = dp[i - 1][j];
            if (j as i32 - weights[i]) >= 0
                && dp[i - 1][j - weights[i] as usize] + values[i] >= dp[i][j]
            {
                dp[i][j] = dp[i - 1][j - weights[i] as usize] + values[i];
            }
        }
    }
    let mut ans = 0;
    let mut ans_j = 0;
    for j in (0..=w).rev() {
        if dp[n][j] > ans {
            ans = dp[n][j];
            ans_j = j;
        }
    }
    println!("{}", ans);
    print_path(&dp, weights, n, ans_j);
}

fn knapsack_solver() {
    let mut lines = include_str!("rucksack.in").lines();
    let (n, w) = {
        let v = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        (v[0], v[1])
    };
    let mut weights = vec![0; n + 1];
    let mut values = vec![0; n + 1];
    for i in 1..=n {
        let v = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        weights[i] = v[0];
        values[i] = v[1];
    }
    problem_knapsack(n, w, &weights, &values);
}

fn problem_lcs(a: &[i32], b: &[i32]) {
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            if a[i - 1] == b[j - 1] {
                dp[i][j] = (dp[i - 1][j - 1] + 1).max(dp[i][j]);
            }
        }
    }

    println!("{}", dp[n][m]);

    let mut i = n;
    let mut j = m;
    let mut ans = vec![];
    while i > 0 && j > 0 {
        if a[i - 1] == b[j - 1] {
            ans.push(a[i - 1]);
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] == dp[i][j] {
            i -= 1;
        } else if dp[i][j - 1] == dp[i][j] {
            j -= 1;
        }
    }

    println!(
        "{}",
        ans.into_iter()
            .rev()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn solver_lcs() {
    let mut lines = include_str!("seq2.in").lines();
    lines.next().unwrap();
    let mut a = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    lines.next().unwrap();
    let mut b = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    problem_lcs(&a, &b);
}

fn problem_binom(n: usize, k: usize) {
    let mut dp = vec![vec![0u128; n + 1]; n + 1];
    for i in 0..=n {
        dp[0][i] = 1;
        dp[i][i] = 1;
    }
    for i in 1..=k {
        for j in 1..=n {
            dp[i][j] = dp[i - 1][j - 1] + dp[i][j - 1];
        }
    }
    println!("{}", dp[k][n]);
}

fn problem_arithmetics(v: &mut [i32], sum: i32) {
    fn backtrack(v: &mut [i32], sum: i32, i: usize, cur: i32, path: &mut String) {
        if sum == cur && i == v.len() {
            println!("{}", path);
            return;
        }
        if i >= v.len() {
            return;
        }

        backtrack(v, sum, i + 1, cur + v[i], &mut format!("{}+{}", path, v[i]));
        backtrack(v, sum, i + 1, cur - v[i], &mut format!("{}-{}", path, v[i]));
    }

    backtrack(v, sum, 1, v[0], &mut format!("{}", v[0]));
}

fn problem_arithmetics2(v: &mut [i32], sum: i32) {
    fn dfs(k: usize, sum: i32, cur: i32, sign: &mut [i32], v: &mut [i32]) -> bool {
        if k == v.len() && sum == cur {
            for (i, v) in v.iter_mut().enumerate() {
                if i >= 1 {
                    print!("{}", if sign[i] == 0 { '-' } else { '+' });
                }
                print!("{}", v);
            }
            return true;
        }
        if k < v.len() {
            sign[k] = 1;
            if dfs(k + 1, sum, cur + v[k], sign, v) {
                return true;
            }
            sign[k] = 0;
            if dfs(k + 1, sum, cur - v[k], sign, v) {
                return true;
            }
        }
        false
    }
    let n = v.len();
    let mut sign = vec![0; n];
    dfs(0, sum, 0, &mut sign, v);
}

fn problem_arithmetics3(v: &mut [i32], sum: i32) {
    fn fenwick_add(tree: &mut [i32], mut k: i32, num: i32) {
        while k < tree.len() as i32 {
            tree[k as usize] += num;
            k += k & -k;
        }
    }
    fn fenwick_query(tree: &mut [i32], mut k: i32) -> i32 {
        let mut ans = 0;
        while k > 0 {
            ans += tree[k as usize];
            k -= k & -k;
        }
        ans
    }

    fn dfs(v: &mut [i32], tree: &mut [i32], sum: i32, cur: i32, i: usize) -> bool {
        if sum == cur {
            for (k, &mut e) in v.iter_mut().enumerate() {
                if e < 0 || k == 0 {
                    print!("{}", e);
                } else {
                    print!("+{}", e);
                }
            }
            return true;
        }
        for j in i..v.len() {
            v[j] *= -1;
            fenwick_add(tree, j as i32 + 1, v[j] * 2);
            let next_cur = fenwick_query(tree, v.len() as i32);
            if dfs(v, tree, sum, next_cur, j + 1) {
                return true;
            }
            v[j] *= -1;
            fenwick_add(tree, j as i32 + 1, v[j] * 2);
        }
        false
    }

    let n = v.len();
    let mut tree = vec![0; n + 1];
    for i in 1..=n {
        fenwick_add(&mut tree, i as i32, v[i - 1]);
    }
    let cur = fenwick_query(&mut tree, n as i32);
    dfs(v, &mut tree, sum, cur, 1);
}

fn problem_arithmetics4(v: &mut [i32], sum: i32) {
    fn print_path(v: &mut [i32], p: &Vec<Vec<i32>>, sum: i32, i: usize) {
        if i > 0 {
            print_path(v, p, sum - p[i][sum as usize] * v[i], i - 1);
            if p[i][sum as usize] < 0 {
                print!("-");
            } else {
                print!("+");
            }
        }
        print!("{}", v[i]);
    }

    const SMAX: usize = 10000;
    let mut dp = vec![vec![0; SMAX * 2]; v.len()];
    let mut p = vec![vec![0; SMAX * 2]; v.len()];
    dp[0][(SMAX as i32 + v[0]) as usize] = 1;

    for i in 0..v.len() - 1 {
        for j in 0..SMAX * 2 {
            if dp[i][j] == 0 {
                continue;
            }
            dp[i + 1][j + v[i + 1] as usize] = dp[i][j];
            p[i + 1][j + v[i + 1] as usize] = 1;
            dp[i + 1][j - v[i + 1] as usize] = dp[i][j];
            p[i + 1][j - v[i + 1] as usize] = -1;
        }
    }

    print_path(v, &p, SMAX as i32 + sum, v.len() - 1);
}

fn solve_arithmetics() {
    let mut lines = include_str!("arithm2.in").lines();
    let mut sum = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut a = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    problem_arithmetics4(&mut a, sum[0]);
}

fn problem_submatrix(matrix: &[Vec<i32>], queries: &[Vec<i32>]) {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut prefix = vec![vec![0i128; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            prefix[i][j] = prefix[i - 1][j] + prefix[i][j - 1] - prefix[i - 1][j - 1]
                + matrix[i - 1][j - 1] as i128;
        }
    }

    println!(
        "{:?}",
        queries
            .iter()
            .map(|q| (q[0] as usize, q[1] as usize, q[2] as usize, q[3] as usize))
            .map(|(x1, x2, y1, y2)| {
                prefix[x2][y2] - prefix[x1 - 1][y2] - prefix[x2][y1 - 1] + prefix[x1 - 1][y1 - 1]
            })
            .sum::<i128>()
    );
}

fn solve_submatrix() {
    let mut lines = include_str!("rectangle.in").lines();

    let (n, _) = {
        let v = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        (v[0], v[1])
    };

    let mut matrix = vec![];
    for _ in 0..n {
        matrix.push(
            lines
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect(),
        );
    }

    let q_num = lines.next().unwrap().parse::<i32>().unwrap();
    let mut queries = vec![];
    for _ in 0..q_num {
        queries.push(
            lines
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect(),
        );
    }

    problem_submatrix(&matrix, &queries);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test2() {
        println!("{:?}", problem2(6, 4));
    }

    #[test]
    fn test3() {
        println!("{:?}", problem3(10, 8643));
    }

    #[test]
    fn test4() {
        problem4()
    }

    #[test]
    fn test5() {
        problem5()
    }

    #[test]
    fn test6() {
        problem6(3);
    }

    #[test]
    fn test7() {
        problem7(7);
    }

    #[test]
    fn test3_4_and_5() {
        problem3_4_and_5();
    }

    #[test]
    fn test3_6() {
        problem3_6();
    }

    #[test]
    fn test3_7() {
        problem_3_7();
    }

    #[test]
    fn test3_8() {
        problem_3_8();
    }

    #[test]
    fn test_dominos() {
        problem_dominos(94);
    }

    #[test]
    fn test_bugs() {
        problem_bug(&mut [
            &mut [1, 4, 1, 2, 3],
            &mut [2, 3, 2, 1, 4],
            &mut [1, 1, 1, 2, 4],
            &mut [2, 5, 1, 7, 1],
        ])
    }

    #[test]
    fn test_knapsack() {
        knapsack_solver();
        // problem_knapsack(3, 12, &[2, 5, 10], &[10, 20, 30]);
    }

    #[test]
    fn test_lcs() {
        solver_lcs();
        // problem_lcs(&[3, 2, 4, 2, 1, 7, 6], &[4, 2, 5, 3, 1, 6, 5, 2, 3]);
    }

    #[test]
    fn test_binom() {
        problem_binom(2, 1);
        problem_binom(3, 2);
        problem_binom(4, 3);
        problem_binom(20, 17);
        problem_binom(50, 20);
    }

    #[test]
    fn test_arithmetic() {
        problem_arithmetics4(&mut [1, 2, 3], 0); // 1 + 2 - 3
        println!();
        solve_arithmetics(); // 21-27+34+20-29-24+38+38-22-24
    }

    #[test]
    fn test_submatrix_sum() {
        problem_submatrix(
            &vec![
                vec![1, 3, 7, -1, 7, 11],
                vec![2, 6, 5, 1, 1, 3],
                vec![-3, 0, 2, 0, 3, 8],
                vec![5, 1, 3, 1, 4, 7],
                vec![6, 1, -2, 2, 1, 0],
            ],
            &vec![vec![2, 3, 2, 3], vec![1, 1, 5, 6], vec![3, 5, 3, 6]],
        );

        solve_submatrix();
    }
}
