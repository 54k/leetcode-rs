use std::error::Error;

fn problem226() -> Result<(), Box<dyn Error>> {
    use std::io;
    let stdin = io::stdin();
    let mut l = String::new();
    stdin.read_line(&mut l)?;
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=l.parse()? {
        let c = (a + b) % 10;
        a = b;
        b = c;
    }
    println!("{}", b);
    Ok(())
}

fn problem228() -> Result<(), Box<dyn Error>> {
    use std::io;
    fn period(m: u128) -> Vec<u128> {
        fn norm(a: u128, m: u128) -> u128 {
            (a % m + m) % m
        }
        fn add(a: u128, b: u128, m: u128) -> u128 {
            norm(norm(a, m) + norm(b, m), m)
        }
        let mut ans = vec![];
        ans.push(0);
        ans.push(1);
        let mut a = 0u128;
        let mut b = 1u128;
        for i in 2..=m * 6 {
            let c = add(a, b, m);
            ans.push(c);
            if i > 2 && ans[i as usize] == 1 && ans[i as usize - 1] == 0 {
                return ans;
            }
            a = b;
            b = c;
        }
        ans
    }
    let stdin = io::stdin();
    let mut l = String::new();
    stdin.read_line(&mut l)?;
    let l = l
        .split(' ')
        .map(|n| n.parse::<u128>().unwrap())
        .collect::<Vec<_>>();

    let n = l[0];
    let m = l[1];
    let p = period(m);
    println!("{}", p[(n % (p.len() as u128 - 2)) as usize]);
    Ok(())
}

fn problem234() {
    use std::io;
    let stdin = io::stdin();
    let mut l = String::new();
    stdin.read_line(&mut l).unwrap();
    let l = l
        .split(' ')
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let a = l[0];
    let b = l[1];
    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            return a;
        }
        gcd(b, (a % b + b) % b)
    }
    println!("{}", gcd(a, b));
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    fn test226() {
        problem226().unwrap();
    }

    // #[test]
    fn test228() {
        problem228().unwrap();
    }
}
