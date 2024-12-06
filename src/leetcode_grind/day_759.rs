// https://leetcode.com/problems/move-pieces-to-obtain-a-string/description/?envType=daily-question&envId=2024-12-05
pub fn can_change(start: String, target: String) -> bool {
    let n = start.len();
    let start = start.chars().collect::<Vec<_>>();
    let target = target.chars().collect::<Vec<_>>();
    let mut v1 = vec![];
    let mut v2 = vec![];
    for i in 0..n {
        if start[i] != '_' {
            v1.push((i, start[i]));
        }
        if target[i] != '_' {
            v2.push((i, target[i]));
        }
    }

    if v1.len() != v2.len() {
        return false;
    }

    for i in 0..v1.len() {
        if v1[i].1 != v2[i].1 {
            return false;
        }

        if v1[i].1 == 'L' && v1[i].0 < v2[i].0 {
            return false;
        }

        if v1[i].1 == 'R' && v1[i].0 > v2[i].0 {
            return false;
        }
    }
    true
}

// https://leetcode.com/problems/maximum-number-of-integers-to-choose-from-a-range-i/description/
pub fn max_count(mut banned: Vec<i32>, n: i32, mut max_sum: i32) -> i32 {
    banned.sort();
    let mut banned_idx = 0;
    let mut count = 0;

    for num in 1..=n {
        if banned_idx < banned.len() && banned[banned_idx] == num {
            while banned_idx < banned.len() && banned[banned_idx] == num {
                banned_idx += 1;
            }
        } else {
            max_sum -= num;
            if max_sum >= 0 {
                count += 1;
            }
        }
    }
    count
}
