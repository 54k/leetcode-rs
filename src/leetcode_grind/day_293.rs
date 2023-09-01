// https://leetcode.com/problems/counting-bits/description/
pub fn count_bits_pop_count_i(n: i32) -> Vec<i32> {
    fn pop_count(mut n: i32) -> i32 {
        let mut ans = 0;
        while n != 0 {
            ans += n & 1;
            n >>= 1;
        }
        ans
    }

    let mut ans = vec![0];
    for i in 1..=n {
        ans.push(pop_count(i));
    }
    ans
}

pub fn count_bits_pop_count_ii(n: i32) -> Vec<i32> {
    fn pop_count(mut x: i32) -> i32 {
        let mut count = 0;
        while x != 0 {
            x &= x - 1;
            count += 1;
        }
        count
    }

    let mut ans = vec![0];
    for i in 1..=n {
        ans.push(pop_count(i));
    }
    ans
}

pub fn count_bits_dp(n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut ans = vec![0; n as usize + 1];
    let mut x = 0;
    let mut b = 1;

    // [0, b) is calculated
    while b <= n {
        // generate [b, 2b) or [b, n) from [0, b)
        while x < b && x + b <= n {
            ans[x + b] = ans[x] + 1;
            x += 1;
        }
        // println!("{:?}", ans);
        x = 0; // reset
        b <<= 1; // b = 2b
    }

    ans
}

// [0, 1b, 0, 0, 0, 0] b = 1, x = 0..
// [0, 1, 1b, 2, 0, 0] b = 2, x = 0..
// [0, 1, 1, 2, 1b, 2] b = 4, x = 0..

#[test]
fn count_bits() {
    let res = count_bits_dp(5);
    println!("{:?}", res);
}
