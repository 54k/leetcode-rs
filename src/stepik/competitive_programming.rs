use std::cmp::Ordering;

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
}
