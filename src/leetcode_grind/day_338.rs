// https://leetcode.com/problems/pascals-triangle-ii
pub fn get_row_bruteforce(row_index: i32) -> Vec<i32> {
    fn get_num(row: i32, col: i32) -> i32 {
        if row == 0 || col == 0 || row == col {
            return 1;
        }
        get_num(row - 1, col - 1) + get_num(row - 1, col)
    }

    let mut ans = vec![];
    for i in 0..=row_index {
        ans.push(get_num(row_index, i));
    }
    ans
}

// https://leetcode.com/problems/pascals-triangle-ii/description
pub fn get_row_mem_optimized_dp(row_index: i32) -> Vec<i32> {
    let mut row = vec![1];
    for i in 0..row_index as usize {
        for j in (1..=i).rev() {
            row[j] += row[j - 1];
        }
        row.push(1);
    }
    row
}
