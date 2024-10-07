// https://leetcode.com/problems/rank-transform-of-an-array/description/?envType=daily-question&envId=2024-10-02
pub fn array_rank_transform(mut arr: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut num_to_rank = HashMap::new();
    let mut sorted_arr = arr.clone();
    sorted_arr.sort();
    let mut rank = 1;
    for i in 0..sorted_arr.len() {
        if i > 0 && sorted_arr[i] != sorted_arr[i - 1] {
            rank += 1;
        }
        num_to_rank.insert(sorted_arr[i], rank);
    }
    for i in 0..arr.len() {
        arr[i] = num_to_rank[&arr[i]];
    }
    arr
}
