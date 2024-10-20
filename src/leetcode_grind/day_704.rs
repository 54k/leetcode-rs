//    https://leetcode.com/problems/parsing-a-boolean-expression/description/?envType=daily-question&envId=2024-10-20
pub fn parse_bool_expr(expression: String) -> bool {
    fn evaluate_sub_expr(op: char, values: Vec<char>) -> char {
        if op == '!' {
            return if values[0] == 't' { 'f' } else { 't' };
        }

        if op == '&' {
            for val in values {
                if val == 'f' {
                    return 'f';
                }
            }
            return 't';
        }

        if op == '|' {
            for val in values {
                if val == 't' {
                    return 't';
                }
            }
            return 'f';
        }
        'f' // should never happen
    }

    let mut st = vec![];
    for curr_char in expression.chars() {
        if curr_char == ')' {
            let mut values = vec![];
            while st.len() > 0 && *st.last().unwrap() != '(' {
                values.push(st.pop().unwrap());
            }
            st.pop();
            let op = st.pop().unwrap();
            let mut result = evaluate_sub_expr(op, values);
            st.push(result);
        } else if curr_char != ',' {
            st.push(curr_char);
        }
    }

    *st.last().unwrap() == 't'
}
