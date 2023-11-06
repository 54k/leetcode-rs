// https://leetcode.com/problems/count-primes/description/
pub fn count_primes(n: i32) -> i32 {
    if n <= 2 {
        return 0;
    }

    let mut sieve = vec![false; n as usize];
    for p in 2..=(n as f64).sqrt() as usize {
        if !sieve[p] {
            for j in (p * p..n as usize).step_by(p) {
                sieve[j] = true;
            }
        }
    }

    let mut primes_count = 0;
    for i in 2..n as usize {
        if !sieve[i] {
            primes_count += 1;
        }
    }
    primes_count
}

// https://leetcode.com/problems/prime-pairs-with-target-sum/
pub fn find_prime_pairs(n: i32) -> Vec<Vec<i32>> {
    if n <= 3 {
        return vec![];
    }

    let n = n as usize;
    let mut primes = vec![false; n];
    for p in 2..=((n as f64).sqrt() as usize) {
        if !primes[p] {
            for j in (p * p..n).step_by(p) {
                primes[j] = true;
            }
        }
    }

    let mut ans = vec![];
    for i in ((n + 1) / 2..=n - 2).rev() {
        if !primes[i] && !primes[n - i] {
            let (f, s) = ((n - i) as i32, i as i32);
            ans.push(vec![f, s]);
        }
    }
    ans
}

// https://leetcode.com/problems/check-if-all-the-integers-in-a-range-are-covered/description/
pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
    let mut r = vec![0; 52];
    for range in ranges {
        r[range[0] as usize] += 1;
        r[range[1] as usize + 1] -= 1;
    }

    for i in 1..r.len() {
        r[i] += r[i - 1];
    }

    for i in left as usize..=right as usize {
        if r[i] == 0 {
            return false;
        }
    }
    true
}
