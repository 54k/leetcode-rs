// https://leetcode.com/problems/delete-characters-to-make-fancy-string/description/?envType=daily-question&envId=2024-11-01
pub fn make_fancy_string(s: String) -> String {
    let s = s.chars().collect::<Vec<_>>();
    let mut ans = String::new();
    ans.push(s[0]);
    if s.len() > 1 {
        ans.push(s[1]);
    }
    for i in 2..s.len() {
        if s[i] == s[i - 1] && s[i - 1] == s[i - 2] {
            continue;
        }
        ans.push(s[i] as char);
    }
    ans
}
