// https://leetcode.com/problems/daily-temperatures/
pub fn daily_temperatures_monostack(temperatures: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![0; temperatures.len()];
    let mut stack = vec![];
    for i in 0..temperatures.len() {
        while !stack.is_empty() && temperatures[stack[stack.len() - 1]] < temperatures[i] {
            let j = stack.pop().unwrap();
            ans[j] = (i - j) as i32;
        }
        stack.push(i);
    }
    ans
}
