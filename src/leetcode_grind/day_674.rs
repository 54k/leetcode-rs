// https://leetcode.com/problems/lexicographical-numbers/description/?envType=daily-question&envId=2024-09-21
pub fn lexical_order(n: i32) -> Vec<i32> {
    let mut res = vec![];
    let mut curr_num = 1;
    for i in 0..n {
        res.push(curr_num);
        if curr_num * 10 <= n {
            curr_num *= 10;
        } else {
            while curr_num % 10 == 9 || curr_num >= n {
                curr_num /= 10;
            }
            curr_num += 1;
        }
    }
    res
}
