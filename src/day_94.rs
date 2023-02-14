// https://leetcode.com/problems/add-binary/description/
pub fn add_binary(a: String, b: String) -> String {
    let a = a.chars().rev().collect::<Vec<_>>();
    let b = b.chars().rev().collect::<Vec<_>>();

    let mut ans = String::new();
    let mut carry = 0;
    let mut i = 0;
    let mut j = 0;

    loop {
        let a_i = if i < a.len() {
            let val = a[i] as u32 - '0' as u32;
            i += 1;
            val
        } else {
            0
        };

        let b_j = if j < b.len() {
            let val = b[j] as u32 - '0' as u32;
            j += 1;
            val
        } else {
            0
        };

        let mut sum = a_i + b_j + carry;
        carry = if sum >= 2 { 1 } else { 0 };
        sum %= 2;
        ans.push(char::from_digit(sum, 2).unwrap());

        if i == a.len() && j == b.len() {
            if carry > 0 {
                ans.push(char::from_digit(carry, 2).unwrap());
            }
            break;
        }
    }
    ans.chars().rev().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test223() {
        println!("{}", add_binary("11".to_string(), "1".to_string())); // 100
        println!("{}", add_binary("1010".to_string(), "1011".to_string())); // 10101
    }
}
