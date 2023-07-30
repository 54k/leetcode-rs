// https://codeforces.com/problemset/problem/996/A
fn solve(mut num: i32) -> i32 {
    const BANKNOTES: [i32; 5] = [100, 20, 10, 5, 1];
    let mut j = 0;
    let mut result = 0;
    while num > 0 {
        while num < BANKNOTES[j] {
            j += 1;
        }

        result += num / BANKNOTES[j];
        num %= BANKNOTES[j];
    }
    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::stdin();
    let mut s = String::new();
    input.read_line(&mut s)?;
    let num = s.trim().parse::<i32>()?;
    println!("{}", solve(num));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_solve() {
        assert_eq!(3, solve(125));
        assert_eq!(5, solve(43));
        assert_eq!(10000000, solve(1000000000));
    }
}
