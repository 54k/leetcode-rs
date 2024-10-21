// https://leetcode.com/problems/split-a-string-into-the-max-number-of-unique-substrings/description/
pub fn max_unique_split(s: String) -> i32 {
    use std::collections::HashSet;
    fn backtrack(
        s: &Vec<char>,
        start: usize,
        seen: &mut HashSet<Vec<char>>,
        count: usize,
        max_count: &mut usize,
    ) {
        if count + (s.len() - start) <= *max_count {
            return;
        }

        if start == s.len() {
            *max_count = (*max_count).max(count);
            return;
        }

        for end in start + 1..=s.len() {
            let ss = &s[start..end];
            if !seen.contains(ss) {
                seen.insert(ss.to_vec());
                backtrack(s, end, seen, count + 1, max_count);
                seen.remove(ss);
            }
        }
    }

    let mut seen = HashSet::new();
    let mut max_count = 0;
    let s = s.chars().collect::<Vec<_>>();
    backtrack(&s, 0, &mut seen, 0, &mut max_count);
    max_count as i32
}
