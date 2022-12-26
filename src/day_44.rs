#[allow(dead_code)]
pub fn multiply(num1: String, num2: String) -> String {
    if num1 == "0" || num2 == "0" {
        return "0".to_string();
    }

    let mut ans = (0..num1.len() + num2.len())
        .map(|_| '0')
        .collect::<Vec<_>>();

    let num1 = num1.chars().rev().collect::<Vec<_>>();
    let num2 = num2.chars().rev().collect::<Vec<_>>();

    for i in 0..num1.len() {
        let digit1 = num1[i] as i32 - '0' as i32;

        for j in 0..num2.len() {
            let digit2 = num2[j] as i32 - '0' as i32;
            let cur_pos = i + j;
            let carry = ans[cur_pos] as i32 - '0' as i32;
            let prod = digit1 * digit2 + carry;

            ans[cur_pos] = ((prod % 10) as u8 + b'0') as char;
            let n = (ans[cur_pos + 1] as u32 - '0' as u32) + ((prod / 10) as u32 + '0' as u32);
            ans[cur_pos + 1] = char::from_u32(n).unwrap();
        }
    }

    while ans[ans.len() - 1] == '0' {
        ans.pop();
    }

    ans.into_iter().rev().collect()
}

// https://leetcode.com/problems/count-and-say/solutions/15995/examples-of-nth-sequence/?orderBy=most_relevant
#[allow(dead_code)]
pub fn count_and_say(n: i32) -> String {
    fn build(result: String) -> String {
        let result = result.chars().collect::<Vec<_>>();

        let mut res = String::new();

        let mut p = 0;
        while p < result.len() {
            let val = result[p];
            let mut count = 0;

            while p < result.len() && result[p] == val {
                p += 1;
                count += 1;
            }

            res.push_str(&count.to_string());
            res.push_str(&val.to_string());
        }

        res
    }

    if n <= 0 {
        return "-1".to_string();
    }

    let mut result = "1".to_string();
    for _ in 1..n {
        result = build(result);
    }

    result
}

#[cfg(test)]
mod test {
    use crate::day_44::*;

    #[test]
    fn test128() {
        println!("{}", multiply("123".to_string(), "456".to_string())); // 56088
        println!("{}", multiply("123".to_string(), "0".to_string())); // 0
    }

    #[test]
    fn test129() {
        println!("{}", count_and_say(4)); // 1 2 1 1
    }
}
