// https://codeforces.com/problemset/problem/996/A
fn solve(v: Vec<i32>) -> i32 {
    let mut ans = 1;
    let mut dp = vec![1; v.len()];
    for i in 1..v.len() {
        if v[i] > v[i - 1] {
            dp[i] += dp[i - 1];
            ans = ans.max(dp[i]);
        }
    }
    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::stdin();
    let mut s = String::new();
    input.read_line(&mut s)?;
    s.clear();
    input.read_line(&mut s)?;
    let v = s
        .trim()
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    println!("{}", solve(v));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(3, solve(vec![1, 7, 2, 11, 15]));
        assert_eq!(6, solve(vec![100, 100, 100, 100, 100, 100]));
        assert_eq!(3, solve(vec![1, 2, 3]));
    }
}
