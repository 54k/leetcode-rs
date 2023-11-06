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
