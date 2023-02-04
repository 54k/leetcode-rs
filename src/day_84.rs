// https://leetcode.com/problems/permutation-in-string/description/
pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }
    let mut h1 = vec![0; 26];
    for c in s1.chars() {
        h1[c as usize - 'a' as usize] += 1;
    }
    for i in 0..=(s2.len() - s1.len()) {
        let mut h2 = vec![0; 26];
        let s2 = s2.chars().collect::<Vec<_>>();
        for j in 0..s1.len() {
            h2[s2[i + j] as usize - 'a' as usize] += 1;
        }
        if h1 == h2 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test195() {
        println!(
            "{}",
            check_inclusion("ab".to_string(), "eidbaooo".to_string())
        ); // true
        println!(
            "{}",
            check_inclusion("ab".to_string(), "eidboaoo".to_string())
        ); // false

        println!(
            "{}",
            check_inclusion("ab".to_string(), "eidboaoo".to_string())
        ); // false
    }
}
