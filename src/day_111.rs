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

// https://leetcode.com/problems/increasing-triplet-subsequence/description/
pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    todo!()
}

// https://leetcode.com/problems/shortest-unsorted-continuous-subarray/description/
pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/min-stack/
struct MinStack {}
impl MinStack {
    fn new() -> Self {
        Self {}
    }

    fn push(&self, val: i32) {}

    fn pop(&self) {}

    fn top(&self) -> i32 {
        todo!()
    }

    fn get_min(&self) -> i32 {
        todo!()
    }
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
