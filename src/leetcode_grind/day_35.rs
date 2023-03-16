#[allow(dead_code)]
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = vec![];
    for t in &tokens {
        let t = t.as_str();
        match t {
            "+" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a + b)
            }
            "-" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a - b);
            }
            "*" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a * b)
            }
            "/" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a / b)
            }
            _ => {
                let mut s = 1;
                let mut num = 0;
                for c in t.to_string().chars() {
                    match c {
                        '-' => s *= -1,
                        _ => {
                            num *= 10;
                            num += c as i32 - '0' as i32;
                        }
                    }
                }
                stack.push(num * s);
            }
        }
    }
    stack.pop().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test119() {
        println!(
            "{}",
            eval_rpn(vec![
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "3".to_string(),
                "*".to_string()
            ])
        ); // 9

        println!(
            "{}",
            eval_rpn(vec![
                "4".to_string(),
                "13".to_string(),
                "5".to_string(),
                "/".to_string(),
                "+".to_string()
            ])
        ); // 6

        println!(
            "{}",
            eval_rpn(vec!["-4".to_string(), "-13".to_string(), "+".to_string(),])
        ); // -17
    }
}
