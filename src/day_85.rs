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

// https://leetcode.com/problems/summary-ranges/
pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.is_empty() {
        return vec![];
    }
    let mut ans = vec![];
    let mut interval = (nums[0], nums[0]);
    for i in 1..nums.len() {
        if nums[i] - nums[i - 1] == 1 {
            interval.1 = nums[i];
        } else {
            ans.push(interval);
            interval = (nums[i], nums[i]);
        }
    }
    ans.push(interval);
    ans.into_iter()
        .map(|interval| {
            if interval.0 == interval.1 {
                format!("{}", interval.0)
            } else {
                format!("{}->{}", interval.0, interval.1)
            }
        })
        .collect()
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

    #[test]
    fn test202() {
        println!("{:?}", summary_ranges(vec![0, 1, 2, 4, 5, 7])); // ["0->2","4->5","7"]
        println!("{:?}", summary_ranges(vec![0, 2, 3, 4, 6, 8, 9])); // ["0","2->4","6","8->9"]
        println!("{:?}", summary_ranges(vec![])); // []
    }
}
