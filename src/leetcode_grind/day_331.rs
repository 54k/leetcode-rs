// https://leetcode.com/problems/valid-number/description/
pub fn is_number_i(s: String) -> bool {
    let s = s.chars().collect::<Vec<_>>();
    let mut seen_digit = false;
    let mut seen_exponent = false;
    let mut seen_dot = false;
    for i in 0..s.len() {
        if char::is_digit(s[i], 10) {
            seen_digit = true;
        } else if s[i] == '-' || s[i] == '+' {
            if i > 0 && s[i - 1] != 'e' && s[i - 1] != 'E' {
                return false;
            }
        } else if s[i] == 'e' || s[i] == 'E' {
            if seen_exponent || !seen_digit {
                return false;
            }
            seen_digit = false;
            seen_exponent = true;
        } else if s[i] == '.' {
            if seen_dot || seen_exponent {
                return false;
            }
            seen_dot = true;
        } else {
            return false;
        }
    }
    seen_digit
}
