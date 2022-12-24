#[allow(dead_code)]
pub fn num_tilings(n: i32) -> i32 {
    fn dfs(n: i32, c: &mut Vec<i32>) -> i32 {
        const MOD: i32 = 1000000007;
        fn mod_add(a: i32, b: i32) -> i32 {
            ((a % MOD) + (b % MOD)) % MOD
        }
        if n == 0 || n == 1 {
            return 1;
        }
        if n == 2 {
            return 2;
        }
        if c[n as usize] > -1 {
            return c[n as usize];
        }
        c[n as usize] = mod_add(dfs(n - 1, c) * 2, dfs(n - 3, c));
        c[n as usize]
    }

    dfs(n, &mut vec![-1; n as usize + 1])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test126() {
        println!("{}", num_tilings(3)); // 5
        println!("{}", num_tilings(1)); // 1
    }
}
