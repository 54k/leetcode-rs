// https://leetcode.com/problems/pascals-triangle/description/
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut ans = vec![vec![1]];
    for row in 1..num_rows as usize {
        let mut next = vec![1];
        for col in 1..ans[row - 1].len() {
            next.push(ans[row - 1][col - 1] + ans[row - 1][col]);
        }
        next.push(1);
        ans.push(next);
    }
    ans
}

// https://leetcode.com/problems/ternary-expression-parser/description/
pub fn parse_ternary(expression: String) -> String {
    let mut stack = vec![];
    let expression = expression.chars().collect::<Vec<_>>();
    let mut i = expression.len() as i32 - 1;

    while i >= 0 {
        let curr = expression[i as usize];
        if curr >= '0' && curr <= '9' || curr == 'T' || curr == 'F' {
            stack.push(curr);
        } else if curr == '?' {
            let on_true = stack.pop().unwrap();
            let on_false = stack.pop().unwrap();
            stack.push(if expression[i as usize - 1] == 'T' {
                on_true
            } else {
                on_false
            });
            i -= 1;
        }

        i -= 1;
    }
    stack.pop().unwrap().to_string()
}

#[test]
fn test_parse_ternary() {
    let res = parse_ternary("T?2:3".to_string());
    println!("{res}");
}
