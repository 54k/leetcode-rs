// https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length/description/
pub fn max_vowels(s: String, k: i32) -> i32 {
    use std::collections::HashSet;
    let vowels: HashSet<char> = HashSet::from(['a', 'e', 'i', 'o', 'u']);
    let s = s.chars().collect::<Vec<_>>();
    let mut vowels_count = 0;
    let mut vowels_max = 0;
    let mut start = 0;
    for end in 0..s.len() {
        if vowels.contains(&s[end]) {
            vowels_count += 1;
        }
        if end >= k as usize {
            if vowels.contains(&s[start]) {
                vowels_count -= 1;
            }
            start += 1;
        }
        vowels_max = vowels_max.max(vowels_count);
    }
    vowels_max
}

// https://leetcode.com/problems/get-equal-substrings-within-budget/description/
pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();
    let cost = s
        .iter()
        .copied()
        .zip(t.clone())
        .map(|(x, y)| (x as i32 - y as i32).abs())
        .collect::<Vec<_>>();

    let mut cur_cost = 0;
    let mut start = 0usize;
    let mut ans = 0;
    for end in 0..s.len() {
        cur_cost += cost[end];
        while cur_cost > max_cost {
            cur_cost -= cost[start];
            start += 1;
        }
        ans = ans.max(end as i32 - start as i32 + 1);
    }
    ans
}

// https://leetcode.com/problems/longest-nice-subarray/description/
pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
    let mut used = 0;
    let mut j = 0;
    let mut ans = 0;
    for i in 0..nums.len() {
        while (nums[i] & used) != 0 {
            used ^= nums[j];
            j += 1;
        }
        used |= nums[i];
        ans = ans.max(i as i32 - j as i32 + 1)
    }
    ans
}

// https://leetcode.com/problems/reverse-prefix-of-word/
pub fn reverse_prefix(word: String, ch: char) -> String {
    let mut word = word.chars().collect::<Vec<_>>();
    let mut i = 0;
    while i < word.len() {
        if word[i] == ch {
            for j in 0..=i / 2 {
                word.swap(j, i - j);
            }
            break;
        }
        i += 1;
    }
    word.into_iter().collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test396() {
        println!("{}", max_vowels("abciiidef".to_string(), 3)); // 3
        println!("{}", max_vowels("leetcode".to_string(), 2)); // 2
    }

    #[test]
    fn test397() {
        println!(
            "{}",
            equal_substring("abcd".to_string(), "bcdf".to_string(), 3)
        ); // 3
    }

    #[test]
    fn test398() {
        println!("{}", longest_nice_subarray(vec![1, 3, 8, 48, 10])); // 3
    }

    #[test]
    fn test399() {
        println!("{}", reverse_prefix("abcdefd".to_string(), 'd'));
    }
}
