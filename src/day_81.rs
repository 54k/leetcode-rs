// https://leetcode.com/problems/greatest-common-divisor-of-strings/solutions/3024822/greatest-common-divisor-of-strings/
pub fn gcd_of_strings(str1: String, str2: String) -> String {
    fn gcd(x: usize, y: usize) -> usize {
        if y == 0 {
            return x;
        }
        gcd(y, x % y)
    }
    if !(format!("{}{}", str1, str2) == format!("{}{}", str2, str1)) {
        return "".to_string();
    }
    let gcd_len = gcd(str1.len(), str2.len());
    str1.chars().take(gcd_len).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test186() {
        println!(
            "{}",
            gcd_of_strings("ABCABC".to_string(), "ABC".to_string())
        ); // ABC

        println!(
            "{}",
            gcd_of_strings("ABABAB".to_string(), "ABAB".to_string())
        ); // AB

        println!("{}", gcd_of_strings("LEET".to_string(), "CODE".to_string())); // ""
    }
}
