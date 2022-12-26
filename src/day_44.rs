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

#[cfg(test)]
mod test {
    use crate::day_44::multiply;

    #[test]
    fn test128() {
        println!("{}", multiply("123".to_string(), "456".to_string())); // 56088
        println!("{}", multiply("123".to_string(), "0".to_string())); // 0
    }
}
