// https://leetcode.com/problems/champagne-tower/description
pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
    let mut arr = vec![vec![0.0; 102]; 102];
    arr[0][0] = poured as f64;
    for r in 0..=query_row as usize {
        for c in 0..=r {
            let q = (arr[r][c] - 1.0) / 2.0;
            if q > 0.0 {
                arr[r + 1][c] += q;
                arr[r + 1][c + 1] += q;
            }
        }
    }

    (1_f64).min(arr[query_row as usize][query_glass as usize])
}
