// https://leetcode.com/problems/minimum-falling-path-sum-ii/description
pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut dp = vec![0; n];

    let mut min1 = 0usize;
    let mut vmin1 = i32::MAX;
    let mut min2 = 0usize;
    let mut vmin2 = i32::MAX;

    for (i, &v) in grid[n - 1].iter().enumerate() {
        if vmin1 > v {
            min2 = min1;
            min1 = i;
            vmin2 = vmin1;
            vmin1 = v;
        } else if vmin2 > v {
            min2 = i;
            vmin2 = v;
        }
    }

    //print!("{min1} {vmin1} {min2} {vmin2}\n");

    for row in (0..n - 1).rev() {
        let mut cmin1 = 0;
        let mut cmin2 = 0;

        let mut cvmin1 = i32::MAX;
        let mut cvmin2 = i32::MAX;

        for col in 0..n {
            let c = grid[row][col] + if col != min1 { vmin1 } else { vmin2 };

            if cvmin1 > c {
                cmin2 = cmin1;
                cmin1 = col;
                cvmin2 = cvmin1;
                cvmin1 = c;
            } else if cvmin2 > c {
                cmin2 = col;
                cvmin2 = c;
            }
        }

        min1 = cmin1;
        vmin1 = cvmin1;
        min2 = cmin2;
        vmin2 = cvmin2;

        //print!("{min1} {vmin1} {min2} {vmin2}\n");
    }
    vmin1
}
