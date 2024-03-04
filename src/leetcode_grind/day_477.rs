// https://leetcode.com/problems/longest-uncommon-subsequence-i/
pub fn find_lu_slength(a: String, b: String) -> i32 {
    if a == b {
        return -1;
    }
    return a.len().max(b.len()) as i32;
}
