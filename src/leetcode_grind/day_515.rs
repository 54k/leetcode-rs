// https://leetcode.com/problems/remove-k-digits/description/
pub fn remove_kdigits(num: String, k: i32) -> String {
    let mut k = k;
    let mut stack = vec![];
    for ch in num.chars() {
        while !stack.is_empty()
            && k > 0
            && (ch as i32 - '0' as i32) < (*stack.last().unwrap() as i32 - '0' as i32)
        {
            stack.pop();
            k -= 1;
        }
        stack.push(ch);
    }
    while (k > 0) {
        stack.pop();
        k -= 1;
    }
    let mut ans = "".to_string();
    for e in stack {
        if ans == "" && e == '0' {
            continue;
        }
        ans.push(e);
    }
    if ans == "" {
        "0".to_string()
    } else {
        ans
    }
}
