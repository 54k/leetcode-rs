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
