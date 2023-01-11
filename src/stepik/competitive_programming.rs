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
}
