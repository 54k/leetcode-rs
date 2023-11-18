// https://leetcode.com/problems/divide-a-string-into-groups-of-size-k/description/
pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
    let s = s.chars().collect::<Vec<_>>();

    let n = s.len() as i32;
    // let glen = n / k + 1.min(n % k);

    let mut res = vec![];
    let mut group = "".to_string();

    for ch in s {
        if group.len() == k as usize {
            res.push(group);
            group = "".to_string();
        }
        group.push(ch);
    }

    while group.len() < k as usize {
        group.push(fill);
    }
    res.push(group);
    res
}

#[test]
fn test_divide_string() {
    let res = divide_string("abcdefghij".to_string(), 3, 'x');
    println!("{:?}", res);
}

// https://leetcode.com/problems/positions-of-large-groups/description/
pub fn large_group_positions_1(s: String) -> Vec<Vec<i32>> {
    let s = s.chars().collect::<Vec<_>>();
    let mut res = vec![];
    let mut last = -1;
    for right in 0..=s.len() {
        if last == -1 || right == s.len() || s[right - 1] != s[right] {
            if right as i32 - last >= 3 {
                res.push(vec![last, right as i32 - 1]);
            }
            last = right as i32;
        }
    }
    res
}

pub fn large_group_positions_2(s: String) -> Vec<Vec<i32>> {
    let s = s.chars().collect::<Vec<_>>();
    let mut res = vec![];
    let mut i = 0;
    for j in 0..s.len() {
        if j == s.len() - 1 || s[j] != s[j + 1] {
            if j - i + 1 >= 3 {
                res.push(vec![i as i32, j as i32]);
            }
            i = j + 1;
        }
    }
    res
}

#[test]
fn test_large_group_position() {
    let res = large_group_positions_1("abcdddeeeeaabbbcd".to_string());
    println!("{:?}", res);
}
