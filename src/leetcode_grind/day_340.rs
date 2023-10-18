// https://leetcode.com/problems/basic-calculator/description/
pub fn calculate(s: String) -> i32 {
    let s = s.chars().collect::<Vec<_>>();

    let mut stack = vec![];
    let mut operand = 0;
    let mut sign = 1;
    let mut result = 0;

    for ch in s {
        if char::is_digit(ch, 10) {
            operand = operand * 10 + char::to_digit(ch, 10).unwrap() as i32;
        } else if ch == '+' || ch == '-' {
            result += operand * sign;

            sign = if ch == '+' { 1 } else { -1 };
            operand = 0;
        } else if ch == '(' {
            stack.push(result);
            stack.push(sign);

            result = 0;
            operand = 0;
            sign = 1;
        } else if ch == ')' {
            result += operand * sign;
            result *= stack.pop().unwrap();
            result += stack.pop().unwrap();

            sign = 1;
            operand = 0;
        }
    }

    result + (operand * sign)
}
