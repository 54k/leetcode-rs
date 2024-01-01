// https://leetcode.com/problems/assign-cookies/description/
pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
    let (mut g, mut s) = (g, s);
    g.sort();
    s.sort();
    let (mut i, mut j) = (0, 0);

    while j < g.len() && i < s.len() {
        if s[i] >= g[j] {
            j += 1;
        }
        i += 1;
    }

    j as i32
}
