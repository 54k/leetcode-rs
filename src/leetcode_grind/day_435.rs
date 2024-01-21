// https://leetcode.com/problems/fizz-buzz/description/
pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut ans = vec![];
    for i in 1..=n {
        if i % 3 == 0 && i % 5 == 0 {
            ans.push("FizzBuzz".to_string());
        } else if i % 3 == 0 {
            ans.push("Fizz".to_string());
        } else if i % 5 == 0 {
            ans.push("Buzz".to_string());
        } else {
            ans.push(format!("{i}"));
        }
    }
    ans
}
