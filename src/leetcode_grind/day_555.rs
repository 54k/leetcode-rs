// https://leetcode.com/problems/count-the-number-of-special-characters-ii/description/
pub fn number_of_special_chars(word: String) -> i32 {
    let mut ans = 0;
    for c in 'a'..='z' {
        let up: Vec<_> = c.to_uppercase().collect();
        let flower = word.rfind(c);
        let fupper = word.find(up[0]);
        if flower.is_some() && fupper.is_some() {
            if flower.unwrap() < fupper.unwrap() {
                ans += 1;
            }
        }
    }
    ans
}
