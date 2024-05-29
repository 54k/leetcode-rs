// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-in-binary-representation-to-one/description/
pub fn num_steps(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();

    let mut ops = 0;
    let mut carry = 0;

    for i in (1..n).rev() {
        let digit = s[i] + carry;
        if digit % 2 == 1 {
            ops += 2;
            carry = 1;
        } else {
            ops += 1;
        }
    }
    ops + carry as i32
}
