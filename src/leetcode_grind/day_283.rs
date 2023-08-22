// https://leetcode.com/problems/excel-sheet-column-title/description/
pub fn convert_to_title(mut column_number: i32) -> String {
    let mut ans = "".to_string();
    while column_number > 0 {
        column_number -= 1;
        ans.push(char::from_u32((column_number % 26) as u32 + 'A' as u32).unwrap());
        column_number /= 26;
    }
    ans.chars().into_iter().rev().collect()
}

// https://leetcode.com/problems/excel-sheet-column-number/description/
pub fn title_to_number(column_title: String) -> i32 {
    use std::collections::HashMap;
    let mut alpha_map = HashMap::new();
    for i in 0..26 {
        let ch = i + 65;
        alpha_map.insert(char::from_u32(ch as u32).unwrap(), i + 1);
    }
    let mut result = 0;
    for (i, ch) in column_title.chars().rev().enumerate() {
        result += alpha_map[&ch] * 26i32.pow(i as u32);
    }
    result
}

pub fn title_to_number2(column_title: String) -> i32 {
    let mut result = 0;
    let column_title = column_title.chars().collect::<Vec<_>>();
    let n = column_title.len();
    for i in 0..n {
        result = result * 26;
        result += (column_title[i] as i32 - 'A' as i32 + 1);
    }
    result
}
