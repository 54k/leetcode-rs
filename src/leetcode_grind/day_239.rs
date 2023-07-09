// https://leetcode.com/problems/substring-with-largest-variance/description/
pub fn largest_variance(s: String) -> i32 {
    let mut counter = vec![0; 26];
    for ch in s.chars() {
        counter[ch as usize - 'a' as usize] += 1;
    }

    let mut global_max = 0;
    for i in 0..26 {
        for j in 0..26 {
            if i == j || counter[i] == 0 || counter[j] == 0 {
                continue;
            }

            let mut major = i;
            let mut minor = j;

            let mut major_count = 0;
            let mut minor_count = 0;

            let mut rest_minor = counter[j];

            for ch in s.chars() {
                let ch = ch as usize - 'a' as usize;

                if ch == major {
                    major_count += 1;
                } else if ch == minor {
                    minor_count += 1;
                    rest_minor -= 1;
                }

                if minor_count > 0 {
                    global_max = global_max.max(major_count - minor_count);
                }

                if major_count < minor_count && rest_minor > 0 {
                    major_count = 0;
                    minor_count = 0;
                }
            }
        }
    }

    global_max
}
