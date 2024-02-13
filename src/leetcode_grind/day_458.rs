// https://leetcode.com/problems/find-first-palindromic-string-in-the-array/
pub fn first_palindrome(words: Vec<String>) -> String {
    fn is_pali(s: &[char]) -> bool {
        let mut i = 0i32;
        let mut j = s.len() as i32 - 1;
        while i <= j {
            if s[i as usize] != s[j as usize] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }

    for w in words {
        if is_pali(&w.clone().chars().collect::<Vec<_>>()) {
            return w;
        }
    }

    "".to_string()
}
