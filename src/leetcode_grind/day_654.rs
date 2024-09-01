// https://leetcode.com/problems/convert-1d-array-into-2d-array/description/?envType=daily-question&envId=2024-09-01
pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
    if m * n != original.len() as i32 {
        return vec![];
    }
    let mut ans = vec![vec![0; n as usize]; m as usize];
    for i in 0..original.len() {
        let r = i / n as usize;
        let c = i % n as usize;
        ans[r][c] = original[i];
    }
    ans
}
