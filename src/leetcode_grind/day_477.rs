// https://leetcode.com/problems/longest-uncommon-subsequence-i/
pub fn find_lu_slength_i(a: String, b: String) -> i32 {
    if a == b {
        return -1;
    }
    return a.len().max(b.len()) as i32;
}

// https://leetcode.com/problems/longest-uncommon-subsequence-ii/description/
pub fn find_lu_slength_ii(strs: Vec<String>) -> i32 {
    fn is_subseq(a: &Vec<char>, b: &Vec<char>) -> bool {
        let mut j = 0;
        for i in 0..b.len() {
            if j < a.len() && a[j] == b[i] {
                j += 1;
            }
        }
        j == a.len()
    }

    let mut res = -1;
    for i in 0..strs.len() {
        let mut good = true;
        for j in 0..strs.len() {
            if i == j {
                continue;
            }
            let a = &strs[i];
            let b = &strs[j];
            if is_subseq(&a.chars().collect(), &b.chars().collect()) {
                good = false;
                break;
            }
        }
        if good {
            res = res.max(strs[i].len() as i32);
        }
    }
    res
}
