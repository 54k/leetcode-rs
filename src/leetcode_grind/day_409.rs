// https://leetcode.com/problems/maximum-length-of-pair-chain/description/
pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
    let mut pairs = pairs;
    pairs.sort_by_key(|x| x[1]);
    let mut ans = 0;
    let mut tail = i32::MIN;
    for pair in pairs {
        if pair[0] > tail {
            ans += 1;
            tail = pair[1];
        }
    }
    ans
}
