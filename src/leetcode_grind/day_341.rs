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

pub fn calculate_no_stack(s: String) -> i32 {
    let n = s.len();

    let mut result = 0;
    let mut curr = 0;
    let mut prev_op = '+';
    let mut stack_top = 0; // prev num

    for (i, ch) in s.chars().enumerate() {
        if char::is_digit(ch, 10) {
            curr = curr * 10 + (ch as i32 - '0' as i32);
        }

        if !char::is_digit(ch, 10) && !char::is_whitespace(ch) || i == n - 1 {
            if prev_op == '+' || prev_op == '-' {
                result += stack_top;
                stack_top = if prev_op == '-' { -curr } else { curr };
            } else if prev_op == '*' {
                stack_top *= curr;
            } else if prev_op == '/' {
                stack_top /= curr;
            }

            prev_op = ch;
            curr = 0;
        }
    }

    result * stack_top
}