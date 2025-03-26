// https://leetcode.com/problems/minimum-operations-to-make-a-uni-value-grid/description/
pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
    let mut nums_array = vec![];
    let mut result = i32::MAX;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] % x != grid[0][0] % x {
                return -1;
            }
            nums_array.push(grid[row][col]);
        }
    }

    nums_array.sort();
    let len = nums_array.len();
    let mut prefix_sum = vec![0; len];
    let mut suffix_sum = vec![0; len];

    for index in 1..len {
        prefix_sum[index] = prefix_sum[index-1] + nums_array[index - 1];
    } 

    for index in (0..len - 1).rev() {
        suffix_sum[index] = suffix_sum[index + 1] + nums_array[index + 1];
    }

    for index in 0..len {
        let left_operations = (nums_array[index] * index as i32 - prefix_sum[index]) / x;
        let right_operations = (suffix_sum[index] - nums_array[index] * (len - index - 1) as i32) / x;

        result = result.min(left_operations + right_operations);
    }

    result
}