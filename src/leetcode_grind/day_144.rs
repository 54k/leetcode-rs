// https://leetcode.com/problems/minimize-maximum-of-array/
// https://leetcode.com/problems/minimize-maximum-of-array/editorial/
pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut prefix_sum = 0i64;
    for i in 0..nums.len() {
        prefix_sum += nums[i] as i64;
        ans = ans.max(((prefix_sum + i as i64) / (i as i64 + 1)) as i32);
    }
    ans
}

// https://leetcode.com/problems/first-letter-to-appear-twice/description/
pub fn repeated_character(s: String) -> char {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    for ch in s.chars() {
        if set.contains(&ch) {
            return ch;
        }
        set.insert(ch);
    }
    unreachable!()
}

// https://leetcode.com/problems/check-if-the-sentence-is-pangram/description/
// https://leetcode.com/problems/check-if-the-sentence-is-pangram/editorial/
pub fn check_if_pangram(sentence: String) -> bool {
    let mut set = 0;
    for ch in sentence.chars() {
        set |= 1 << (ch as i32 - 'a' as i32);
    }
    set == (1 << 26) - 1
}

// https://leetcode.com/problems/missing-number/editorial/
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut missing = nums.len() as i32;
    for i in 0..nums.len() {
        missing ^= i as i32 ^ nums[i];
    }
    missing
}

// https://leetcode.com/problems/counting-elements/description/
// https://leetcode.com/problems/counting-elements/editorial/
pub fn count_elements(arr: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    for i in 0..arr.len() {
        set.insert(arr[i]);
    }
    let mut count = 0;
    for i in 0..arr.len() {
        if set.contains(&(arr[i] + 1)) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test400() {
        println!("{}", minimize_array_value(vec![3, 7, 1, 6])); // 5
    }

    #[test]
    fn test401() {
        println!("{}", repeated_character("abccbaacz".to_string())); // c
    }

    #[test]
    fn test402() {
        println!(
            "{}",
            check_if_pangram("thequickbrownfoxjumpsoverthelazydog".to_string())
        ); // true
    }

    #[test]
    fn test403() {
        println!("{}", count_elements(vec![1, 2, 3])); // 2
    }
}
