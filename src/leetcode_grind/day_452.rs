// https://leetcode.com/problems/sort-characters-by-frequency/description
pub fn frequency_sort(s: String) -> String {
    let mut s = s.chars().collect::<Vec<_>>();
    s.sort();

    let mut next = vec![];
    let mut cur = "".to_string();

    for i in 0..s.len() {
        cur.push(s[i]);
        if i == s.len() - 1 || s[i] != s[i + 1] {
            next.push(cur.clone());
            cur.clear();
        }
    }

    next.sort_by_key(|x| x.len());
    next.reverse();

    let mut ans = "".to_string();
    for e in next {
        ans.push_str(e.as_str());
    }
    ans
}
