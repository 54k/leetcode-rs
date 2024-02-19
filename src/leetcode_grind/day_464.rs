// https://leetcode.com/problems/power-of-two/description/
pub fn is_power_of_two(n: i32) -> bool {
    let n = n as i64;
    if n == 0 {
        return false;
    }
    (n & (n - 1)) == 0
}

// https://leetcode.com/problems/string-matching-in-an-array/
pub fn string_matching(words: Vec<String>) -> Vec<String> {
    let mut words = words;
    words.sort_by_key(|x| x.len());
    let n = words.len();
    let mut ans = vec![];
    for i in 0..n {
        let w1 = &words[i];
        for j in i + 1..n {
            let w2 = &words[j];
            if w2.contains(w1) {
                ans.push(w1.clone());
                break;
            }
        }
    }
    ans
}
