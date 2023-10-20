// https://leetcode.com/problems/basic-calculator-iii/
pub fn calculate(s: String) -> i32 {
    fn eval(operator: &str, x: &str, y: &str) -> String {
        let result: i32 = match operator {
            "+" => x.parse::<i32>().unwrap(),
            "-" => -(x.parse::<i32>().unwrap()),
            "*" => x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap(),
            _ => x.parse::<i32>().unwrap() / y.parse::<i32>().unwrap(),
        };
        return result.to_string();
    }

    let mut s = s;
    s.push('@');
    let mut stack = vec![];

    let mut cur_num = "".to_string();
    let mut operator = "+".to_string();

    for ch in s.chars() {
        if char::is_digit(ch, 10) {
            cur_num.push(ch);
        } else if ch == '(' {
            stack.push(operator);
            operator = "+".to_string();
        } else {
            if operator == "*".to_string() || operator == "/".to_string() {
                let x = stack.pop().unwrap();
                stack.push(eval(&operator, &x, &cur_num));
            } else {
                stack.push(eval(&operator, &cur_num, "0"));
            }

            operator = ch.to_string();
            cur_num = "".to_string();

            if ch == ')' {
                let mut res = 0;
                while let Some(top) = stack.pop() {
                    if ["+", "-", "*", "/"].contains(&top.as_str()) {
                        operator = top;
                        break;
                    }
                    res += top.parse::<i32>().unwrap();
                }
                cur_num = res.to_string();
            }
        }
    }

    let mut result = 0;
    while let Some(num) = stack.pop() {
        result += num.parse::<i32>().unwrap();
    }

    result
}

#[test]
fn test_calc() {
    let res = calculate("2*(5+5*2)".to_string());
    println!("{res}");
}
