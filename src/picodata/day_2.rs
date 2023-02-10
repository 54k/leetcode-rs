fn check_parenthesis(s: String) -> bool {
    let mut stack = vec![];
    for c in s.chars() {
        match c {
            '(' => {
                stack.push(')');
            }
            '{' => {
                stack.push('}');
            }
            '[' => {
                stack.push(']');
            }
            ')' | '}' | ']' => {
                if let Some(e) = stack.pop() {
                    if e != c {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}

/*# Capture the flag

// https://www.notion.so/Capture-the-flag-a5bdf847551f4aeaa39278c14990a29c

<aside>
💡 Собеседование на позицию Solution Engineer
</aside>

**Легенда:** Некоторое время назад подрядчик разработал для нас суперкалькулятор `supercalc`.
Бухгалтеры им успешно пользовались и считали зарплаты, но вчера "*всё сломалось, ничего не работает*".

**Задача:** диагностировать проблемы и выдать рекомендации по их устранению.*/
fn capture_the_flag() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_parenthesis() {
        println!("{}", check_parenthesis("([()]{()})".to_string())); // true
        println!("{}", check_parenthesis("[(12()()()()32)]{}".to_string())); // true
        println!("{}", check_parenthesis("[(])".to_string())); // false
        println!("{}", check_parenthesis("123(2[3]1)3".to_string())); // true
        println!(
            "{}",
            check_parenthesis("123dasd!@(2[3()]1)3dzz___31299000()".to_string())
        ); // true
        println!("{}", check_parenthesis("]".to_string())); // false
    }
}
