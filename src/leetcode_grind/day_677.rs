// https://leetcode.com/problems/find-the-length-of-the-longest-common-prefix/description/?envType=daily-question&envId=2024-09-24
pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let mut all_prefixes = HashSet::new();
    for mut val in arr1 {
        while !all_prefixes.contains(&val) && val > 0 {
            all_prefixes.insert(val);
            val /= 10;
        }
    }
    let mut longest_prefix = 0;
    for mut val in arr2 {
        while !all_prefixes.contains(&val) && val > 0 {
            val /= 10;
        }
        if val > 0 {
            longest_prefix = longest_prefix.max(format!("{val}").len() as i32);
        }
    }
    longest_prefix
}

// https://leetcode.com/problems/longest-word-in-dictionary-through-deleting/description/
pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
    fn is_subsequence(x: &Vec<char>, y: &Vec<char>) -> bool {
        let mut j = 0;
        let mut i = 0;
        while i < y.len() && j < x.len() {
            if y[i] == x[j] {
                j += 1;
            }
            i += 1;
        }
        j == x.len()
    }

    let mut ans = vec![];
    let s = s.chars().collect::<Vec<_>>();
    let d = dictionary
        .iter()
        .map(|w| w.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for w in d {
        if is_subsequence(&w, &s) {
            if w.len() > ans.len() || (w.len() == ans.len() && w < ans) {
                ans = w;
            }
        }
    }
    ans.iter().collect()
}
