// https://leetcode.com/problems/largest-3-same-digit-number-in-string/description
pub fn largest_good_integer(num: String) -> String {
    let num = num.chars().collect::<Vec<_>>();

    let mut max = "".to_string();

    for i in 0..num.len() - 2 {
        let a = num[i];
        let b = num[i + 1];
        let c = num[i + 2];

        if a == b && b == c {
            max = max.max(format!("{}{}{}", a, b, c));
        }
    }

    max
}
