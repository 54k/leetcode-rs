// https://leetcode.com/problems/minimum-number-of-pushes-to-type-word-ii/description/
pub fn minimum_pushes(word: String) -> i32 {
    let word = word.chars().collect::<Vec<_>>();
    let mut cnt = vec![0; 26];
    for &c in &word {
        cnt[c as usize - 'a' as usize] += 1;
    }
    cnt.sort();
    cnt.reverse();
    let mut ans = 0;
    for i in 0..26 {
        if cnt[i] == 0 {
            break;
        }
        ans += (i / 8 + 1) as i32 * cnt[i];
    }
    ans as i32
}
