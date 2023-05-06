// https://leetcode.com/problems/number-of-subsequences-that-satisfy-the-given-sum-condition/description/
pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
    const MOD: i64 = 1000000007;
    nums.sort();
    let mut power = vec![1; nums.len()];
    for i in 1..nums.len() {
        power[i] = (power[i - 1] * 2) as i64 % MOD;
    }
    let (mut i, mut j) = (0, nums.len() as i32 - 1);
    let mut ans = 0;
    while i <= j {
        if nums[i as usize] + nums[j as usize] <= target {
            ans += power[j as usize - i as usize];
            ans %= MOD;
            i += 1;
        } else {
            j -= 1;
        }
    }
    ans as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test491() {
        println!("{}", num_subseq(vec![1], 1));
    }
}
