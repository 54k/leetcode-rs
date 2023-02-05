// https://leetcode.com/problems/find-all-anagrams-in-a-string/
pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    if p.len() > s.len() {
        return vec![];
    }
    let mut ans = vec![];
    let mut h1 = vec![0; 26];
    for c in p.chars() {
        h1[c as usize - 'a' as usize] += 1;
    }

    let s = s.chars().collect::<Vec<_>>();
    for i in 0..=(s.len() - p.len()) {
        let mut h2 = vec![0; 26];
        for j in 0..p.len() {
            h2[s[i + j] as usize - 'a' as usize] += 1;
        }
        if h1 == h2 {
            ans.push(i as i32);
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test201() {
        println!(
            "{:?}",
            find_anagrams("cbaebabacd".to_string(), "abc".to_string())
        ); // [0, 6]
        println!("{:?}", find_anagrams("abab".to_string(), "ab".to_string())); // [0, 1, 2]
    }
}
