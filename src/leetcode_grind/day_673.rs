// https://leetcode.com/problems/shortest-palindrome/description/?envType=daily-question&envId=2024-09-20
pub fn shortest_palindrome(s: String) -> String {
    fn build_prefix_table(s: String) -> Vec<usize> {
        let s = s.chars().collect::<Vec<_>>();
        let mut prefix_table = vec![0; s.len()];
        let mut length = 0;

        for i in 1..s.len() {
            while length > 0 && s[i] != s[length] {
                length = prefix_table[length - 1];
            }
            if s[i] == s[length] {
                length += 1;
            }
            prefix_table[i] = length;
        }

        prefix_table
    }

    let rev_s = s.chars().rev().collect::<String>();
    let comb_s = s.clone() + "#" + &rev_s;
    let mut prefix_table = build_prefix_table(comb_s.clone());
    let palindrome_length = prefix_table[comb_s.len() - 1];
    let suffix = s[palindrome_length..].chars().rev().collect::<String>();
    suffix + &s
}
