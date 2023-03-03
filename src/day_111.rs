// https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/editorial/
pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack = haystack.chars().collect::<Vec<_>>();
    let needle = needle.chars().collect::<Vec<_>>();
    for (i, &ch) in haystack.iter().enumerate() {
        if ch == needle[0] {
            let mut j = 0;
            while (i + j) < haystack.len() && j < needle.len() && needle[j] == haystack[i + j] {
                j += 1;
            }
            if j == needle.len() {
                return i as i32;
            }
        }
    }
    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test317() {
        println!("{}", str_str("sadbutsad".to_string(), "sad".to_string())); // 0
        println!("{}", str_str("sadbutsad".to_string(), "but".to_string())); // 3
        println!("{}", str_str("leetcode".to_string(), "leeto".to_string())); // -1
        println!("{}", str_str("aaa".to_string(), "aaaa".to_string())); // -1
    }
}
