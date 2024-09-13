// https://leetcode.com/problems/xor-queries-of-a-subarray/description/?envType=daily-question&envId=2024-09-13
pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = arr.len();
    let mut prefix_xor = vec![0; n + 1];
    for i in 0..n {
        prefix_xor[i + 1] = prefix_xor[i] ^ arr[i];
    }

    let mut result = vec![0; queries.len()];
    for i in 0..queries.len() {
        result[i] = prefix_xor[queries[i][1] as usize + 1] ^ prefix_xor[queries[i][0] as usize];
    }
    result
}
