// https://leetcode.com/problems/check-if-all-as-appears-before-all-bs/description/
pub fn check_string(s: String) -> bool {
    let s = s.chars().collect::<Vec<_>>();
    let mut last_b = -1;
    let mut last_a = -1;

    for i in 0..s.len() {
        let c = s[i];

        if c == 'a' {
            last_a = i as i32;
        } else {
            last_b = i as i32;
        }

        if last_a != -1 && last_b != -1 && last_a > last_b {
            return false;
        }
    }

    true
}

// https://leetcode.com/problems/minimum-deletions-to-make-string-balanced/description/?envType=daily-question&envId=2024-07-30
pub fn minimum_deletions_i(s: String) -> i32 {
    let n = s.len();
    let s = s.chars().collect::<Vec<_>>();
    let mut count_a = vec![0; n];
    let mut count_b = vec![0; n];
    let mut b_count = 0;

    for i in 0..n {
        count_b[i] = b_count;
        if s[i] == 'b' {
            b_count += 1;
        }
    }

    let mut a_count = 0;
    for i in (0..n).rev() {
        count_a[i] = a_count;
        if s[i] == 'a' {
            a_count += 1;
        }
    }

    let mut min_deletions = n;
    for i in 0..n {
        min_deletions = min_deletions.min(count_a[i] + count_b[i]);
    }
    min_deletions as i32
}

pub fn minimum_deletions_ii(s: String) -> i32 {
    let n = s.len();
    let s = s.chars().collect::<Vec<_>>();
    let mut a_count = 0;

    for i in 0..n {
        if s[i] == 'a' {
            a_count += 1;
        }
    }

    let mut b_count = 0;
    let mut min_deletions = n;

    for i in 0..n {
        if s[i] == 'a' {
            a_count -= 1;
        }
        min_deletions = min_deletions.min(a_count + b_count);
        if s[i] == 'b' {
            b_count += 1;
        }
    }

    min_deletions as i32
}

pub fn minimum_deletions_iii(s: String) -> i32 {
    let n = s.len();
    let s = s.chars().collect::<Vec<_>>();
    let mut st = vec![];
    let mut delete_count = 0;

    for i in 0..n {
        if !st.is_empty() && st[st.len() - 1] == 'b' && s[i] == 'a' {
            st.pop();
            delete_count += 1;
        } else {
            st.push(s[i]);
        }
    }

    delete_count
}

pub fn minimum_deletions_iv(s: String) -> i32 {
    let n = s.len();
    let s = s.chars().collect::<Vec<_>>();
    let mut b_count = 0;
    let mut min_deletions = 0;

    for i in 0..n {
        if s[i] == 'b' {
            b_count += 1;
        } else {
            min_deletions = b_count.min(min_deletions + 1);
        }
    }

    min_deletions
}
