// https://leetcode.com/problems/string-compression-iii/description/?envType=daily-question&envId=2024-11-04
pub fn compressed_string(word: String) -> String {
    let word = word.chars().collect::<Vec<_>>();
    let mut res = String::new();
    let mut pos = 0;
    while pos < word.len() {
        let mut cnt = 0;
        let c = word[pos];
        while pos < word.len() && cnt < 9 && c == word[pos] {
            cnt += 1;
            pos += 1;
        }
        res.push_str(&format!("{cnt}"));
        res.push(c);
    }
    res
}
