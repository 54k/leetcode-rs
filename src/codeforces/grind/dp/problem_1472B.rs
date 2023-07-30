// https://codeforces.com/problemset/problem/996/A
fn solve(v: Vec<i32>) -> bool {
    let mut cnt1 = 0;
    let mut cnt2 = 0;
    for i in 0..v.len() {
        if v[i] == 1 {
            cnt1 += 1;
        } else {
            cnt2 += 1;
        }
    }
    if (cnt1 + 2 * cnt2) % 2 != 0 {
        return false;
    } else {
        let sum = (cnt1 + 2 * cnt2) / 2;
        if sum % 2 == 0 || (sum % 2 == 1 && cnt1 != 0) {
            return true;
        }
        return false;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::stdin();
    let mut s = String::new();
    input.read_line(&mut s)?;
    let t = s.trim().parse::<i32>()?;
    for _ in 0..t {
        s.clear();
        input.read_line(&mut s)?;
        input.read_line(&mut s)?;
        s.clear();
        let v = s
            .trim()
            .split(' ')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let res = solve(v);
        if res {
            println!("YES");
        } else {
            println!("NO");
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        println!("{}", solve(vec![1, 1]));
    }
}
