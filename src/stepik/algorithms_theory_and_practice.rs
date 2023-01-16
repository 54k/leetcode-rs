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

struct MaxQueue(Vec<i32>);
impl MaxQueue {
    fn new() -> Self {
        Self(vec![0])
    }
    fn insert(&mut self, val: i32) {
        self.0.push(val);
        self.sift_up();
    }
    fn extract_max(&mut self) -> i32 {
        let last = self.0.len() - 1;
        let res = self.0[1];
        self.0.swap(1, last);
        self.0.pop();
        self.sift_down();
        res
    }
    fn sift_up(&mut self) {
        let mut k = self.0.len() - 1;
        while k / 2 >= 1 && self.0[k] > self.0[k / 2] {
            self.0.swap(k, k / 2);
            k /= 2;
        }
    }
    fn sift_down(&mut self) {
        let mut k = 1;
        let n = self.0.len() - 1;
        while k * 2 <= n {
            let mut j = k * 2;
            if j < n && self.0[j] < self.0[j + 1] {
                j += 1;
            }
            if self.0[k] >= self.0[j] {
                break;
            }
            self.0.swap(k, j);
            k = j;
        }
    }
}

fn problem_pq_max() {
    use std::io;

    macro_rules! parse_line {
    ($($t: ty),+) => ({
        let mut a_str = String::new();
        io::stdin().read_line(&mut a_str).expect("read error");
        let mut a_iter = a_str.split_whitespace();
        (
            $(
            a_iter.next().unwrap().parse::<$t>().ok().unwrap_or_default(),
            )+
        )
    })
}
    let mut max_queue = MaxQueue::new();
    let (n,) = parse_line!(usize);
    for _ in 0..n {
        let (s, val) = parse_line!(String, i32);
        match s.as_str() {
            "Insert" => max_queue.insert(val),
            "ExtractMax" => println!("{}", max_queue.extract_max()),
            _ => panic!(""),
        }
    }
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

    #[test]
    fn test229() {
        // problem_pq_max();

        let mut max_queue = MaxQueue::new();
        max_queue.insert(2);
        max_queue.insert(3);
        max_queue.insert(18);
        max_queue.insert(15);
        max_queue.insert(18);
        max_queue.insert(12);
        max_queue.insert(12);
        max_queue.insert(2);
        println!("{}", max_queue.extract_max());
        println!("{}", max_queue.extract_max());
        println!("{}", max_queue.extract_max());
        println!("{}", max_queue.extract_max());
        println!("{}", max_queue.extract_max());
        println!("{}", max_queue.extract_max());
        println!("{}", max_queue.extract_max());
        println!("{}", max_queue.extract_max());
    }
}
