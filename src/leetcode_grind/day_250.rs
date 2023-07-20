// https://leetcode.com/problems/minimum-window-substring/description/
pub fn min_window_tle(s: String, t: String) -> String {
    use std::collections::HashMap;

    fn is_valid(a: &HashMap<char, i32>, b: &HashMap<char, i32>) -> bool {
        for k in b.keys() {
            if !a.contains_key(k) {
                return false;
            }
            if a[k] < b[k] {
                return false;
            }
        }
        true
    }

    let mut t_map = HashMap::new();
    for ch in t.chars() {
        *t_map.entry(ch).or_insert(0) += 1;
    }

    let mut window = HashMap::new();
    let s = s.chars().collect::<Vec<_>>();

    let (mut sz, mut st) = (usize::MAX, "".to_string());

    let mut left = 0;
    for right in 0..s.len() {
        *window.entry(s[right]).or_insert(0) += 1;

        while left <= right && is_valid(&window, &t_map) {
            if sz > right - left + 1 {
                st = s[left..=right].iter().copied().collect::<String>();
                sz = right - left + 1;
            }

            *window.entry(s[left]).or_insert(0) -= 1;
            if window[&s[left]] == 0 {
                window.remove(&s[left]);
            }
            left += 1;
        }
    }

    st
}

pub fn min_window(s: String, t: String) -> String {
    use std::collections::HashMap;

    let mut t_map = HashMap::new();
    for ch in t.chars() {
        *t_map.entry(ch).or_insert(0) += 1;
    }

    let required = t_map.len();
    let mut formed = 0;

    let mut window = HashMap::new();
    let s = s.chars().collect::<Vec<_>>();

    let (mut sz, mut st) = (usize::MAX, "".to_string());

    let mut left = 0;
    for right in 0..s.len() {
        *window.entry(s[right]).or_insert(0) += 1;

        if t_map.contains_key(&s[right])
            && *window.get(&s[right]).unwrap() == *t_map.get(&s[right]).unwrap()
        {
            formed += 1;
        }

        while left <= right && formed == required {
            if sz > right - left + 1 {
                st = s[left..=right].iter().copied().collect::<String>();
                sz = right - left + 1;
            }

            *window.entry(s[left]).or_insert(0) -= 1;

            if t_map.contains_key(&s[left])
                && *window.get(&s[left]).unwrap() < *t_map.get(&s[left]).unwrap()
            {
                formed -= 1;
            }

            left += 1;
        }
    }

    st
}
