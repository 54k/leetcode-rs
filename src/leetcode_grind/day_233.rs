// https://leetcode.com/problems/buddy-strings/
pub fn buddy_strings(s: String, goal: String) -> bool {
    if s.len() != goal.len() {
        return false;
    }

    if s == goal {
        let mut freq = vec![0; 26];
        for ch in s.chars() {
            freq[ch as usize - 'a' as usize] += 1;
            if freq[ch as usize - 'a' as usize] > 1 {
                return true;
            }
        }
        return false;
    }

    let (mut fi, mut si) = (-1, -1);
    let (s, goal) = (
        s.chars().collect::<Vec<char>>(),
        goal.chars().collect::<Vec<char>>(),
    );

    for i in 0..s.len() {
        if s[i] != goal[i] {
            if fi == -1 {
                fi = i as i32;
            } else if si == -1 {
                si = i as i32;
            } else {
                // We have at least 3 indices with different characters,
                // thus, we can never make the strings equal with only one swap.
                return false;
            }
        }
    }

    if si == -1 {
        // We can't swap if the character at only one index is different.
        return false;
    }

    s[fi as usize] == goal[si as usize] && s[si as usize] == goal[fi as usize]
}

// https://leetcode.com/problems/check-if-one-string-swap-can-make-strings-equal/
pub fn are_almost_equal(s1: String, s2: String) -> bool {
    if s1 == s2 {
        return true;
    }

    let (s1, s2) = (
        s1.chars().collect::<Vec<char>>(),
        s2.chars().collect::<Vec<char>>(),
    );
    let (mut fi, mut si) = (-1, -1);
    for i in 0..s1.len() {
        if s1[i] != s2[i] {
            if fi == -1 {
                fi = i as i32;
            } else if si == -1 {
                si = i as i32;
            } else {
                return false;
            }
        }
    }

    if si == -1 {
        return false;
    }

    s1[fi as usize] == s2[si as usize] && s1[si as usize] == s2[fi as usize]
}
