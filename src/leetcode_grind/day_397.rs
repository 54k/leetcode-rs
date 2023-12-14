// https://leetcode.com/problems/strobogrammatic-number/description/
pub fn is_strobogrammatic_1(num: String) -> bool {
    let mut sub = vec![
        Some('0'),
        Some('1'),
        None,
        None,
        None,
        None,
        Some('9'),
        None,
        Some('8'),
        Some('6'),
    ];
    let mut flip = "".to_string();

    for ch in num.chars().rev() {
        if let Some(n) = sub[ch as usize - '0' as usize] {
            flip.push(n);
        } else {
            return false;
        }
    }
    flip == num
}

pub fn is_strobogrammatic_2(num: String) -> bool {
    let mut sub = vec![
        Some('0'),
        Some('1'),
        None,
        None,
        None,
        None,
        Some('9'),
        None,
        Some('8'),
        Some('6'),
    ];

    let num = num.chars().collect::<Vec<_>>();
    let (mut left, mut right) = (0, num.len() as i32 - 1);

    while left <= right {
        let lch = num[left as usize];
        let rch = num[right as usize];

        left += 1;
        right -= 1;

        if sub[lch as usize - '0' as usize].is_none()
            || sub[lch as usize - '0' as usize].unwrap() != rch
        {
            return false;
        }
    }

    return true;
}
