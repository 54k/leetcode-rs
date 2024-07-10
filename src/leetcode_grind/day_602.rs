// https://leetcode.com/problems/crawler-log-folder/description/?envType=daily-question&envId=2024-07-10
pub fn min_operations_1(logs: Vec<String>) -> i32 {
    let mut folder_depth = 0;
    for current_operation in logs {
        if current_operation == "../" {
            folder_depth = 0.max(folder_depth - 1);
        } else if (current_operation != "./") {
            folder_depth += 1;
        }
    }
    folder_depth
}
