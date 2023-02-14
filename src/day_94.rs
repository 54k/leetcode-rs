// https://leetcode.com/problems/add-binary/description/
pub fn add_binary(a: String, b: String) -> String {
    let a = a.chars().rev().collect::<Vec<_>>();
    let b = b.chars().rev().collect::<Vec<_>>();

    let mut ans = String::new();
    let mut carry = 0;
    let mut i = 0;
    let mut j = 0;

    loop {
        let a_i = if i < a.len() {
            let val = a[i] as u32 - '0' as u32;
            i += 1;
            val
        } else {
            0
        };

        let b_j = if j < b.len() {
            let val = b[j] as u32 - '0' as u32;
            j += 1;
            val
        } else {
            0
        };

        let mut sum = a_i + b_j + carry;
        carry = if sum >= 2 { 1 } else { 0 };
        sum %= 2;
        ans.push(char::from_digit(sum, 2).unwrap());

        if i == a.len() && j == b.len() {
            if carry > 0 {
                ans.push(char::from_digit(carry, 2).unwrap());
            }
            break;
        }
    }
    ans.chars().rev().collect()
}

// https://leetcode.com/problems/score-after-flipping-matrix/
pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
    // flip rows with 0s at 1st pos
    // flip columns with 0s count > that 1s count
    fn flip_row(r: usize, grid: &mut [Vec<i32>]) {
        for i in 0..grid[0].len() {
            if grid[r][i] == 0 {
                grid[r][i] = 1
            } else {
                grid[r][i] = 0
            }
        }
    }
    fn flip_column(c: usize, grid: &mut Vec<Vec<i32>>) {
        for i in 0..grid.len() {
            if grid[i][c] == 0 {
                grid[i][c] = 1
            } else {
                grid[i][c] = 0
            }
        }
    }

    let n = grid.len(); // rows
    let m = grid[0].len(); // cols
    let mut col_zeros = vec![0; m];

    for i in 0..n {
        if grid[i][0] == 0 {
            flip_row(i, &mut grid);
        }
        for j in 0..m {
            if grid[i][j] == 0 {
                col_zeros[j] += 1;
            }
        }
    }
    for i in 0..m {
        if col_zeros[i] > n / 2 {
            flip_column(i, &mut grid);
        }
    }

    let mut ans = 0;
    for i in 0..n {
        let mut row_num = 0;
        for j in 0..m {
            let g_val = grid[i][j] << (m - 1 - j);
            row_num |= g_val;
        }
        ans += row_num;
        // println!("{:?}", grid[i]);
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test223() {
        println!("{}", add_binary("11".to_string(), "1".to_string())); // 100
        println!("{}", add_binary("1010".to_string(), "1011".to_string())); // 10101
    }

    #[test]
    fn test224() {
        println!(
            "{}",
            matrix_score(vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]])
        ); // 39
        println!("{}", matrix_score(vec![vec![0, 1], vec![1, 1]])); // 5
        println!("{}", matrix_score(vec![vec![1, 1], vec![1, 1], vec![0, 1]])); // 8
    }
}
