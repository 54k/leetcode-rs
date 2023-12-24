// https://leetcode.com/problems/minimum-changes-to-make-alternating-binary-string/description/
pub fn min_operations(s: String) -> i32 {
    let mut start0 = 0;
    for (i, ch) in s.chars().enumerate() {
        if i % 2 == 0 {
            if ch == '1' {
                start0 += 1;
            }
        } else {
            if (ch == '0') {
                start0 += 1;
            }
        }
    }

    start0.min(s.len() as i32 - start0)
}
