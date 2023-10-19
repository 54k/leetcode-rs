// https://leetcode.com/problems/basic-calculator-ii/description/
pub fn calculate_with_stack(s: String) -> i32 {
    let mut stack = vec![];
    let mut current = 0;
    let mut prev_operator = '+';

    fn eval(op: char, x: i32, y: i32) -> i32 {
        match op {
            '+' => return x,
            '-' => return -x,
            '*' => return x * y,
            '/' => return x / y,
            _ => panic!(),
        }
    }

    let mut s = s;
    s.push('@');

    for ch in s.chars() {
        if ch == ' ' {
            continue;
        }
        if char::is_digit(ch, 10) {
            current = current * 10 + (ch as i32 - '0' as i32);
        } else {
            if prev_operator == '*' || prev_operator == '/' {
                let x = stack.pop().unwrap();
                stack.push(eval(prev_operator, x, current));
            } else {
                stack.push(eval(prev_operator, current, 0));
            }

            prev_operator = ch;
            current = 0;
        }
    }

    stack.into_iter().sum()
}
