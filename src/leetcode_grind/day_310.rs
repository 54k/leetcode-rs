pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let m = mat.len();
    let n = mat[0].len();
    let mut k = k;

    let mut done = vec![false; m];
    let mut ans = vec![];

    for i in 0..=n {
        for j in 0..m {
            if k == 0 {
                break;
            }
            if done[j] {
                continue;
            }
            if i == n || mat[j][i] == 0 {
                k -= 1;
                done[j] = true;
                ans.push(j as i32);
            }
        }
    }

    ans
}

#[test]
fn test_weakest_rows() {
    let res = k_weakest_rows(
        vec![
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 1],
        ],
        3,
    );
    println!("{:?}", res);
}
