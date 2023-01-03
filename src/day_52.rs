#[allow(dead_code)]
pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let strs: Vec<Vec<char>> = strs
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();
    let mut ans = 0;
    for j in 0..strs[0].len() {
        for i in 1..strs.len() {
            if strs[i][j] < strs[i - 1][j] {
                ans += 1;
                break;
            }
        }
    }
    ans
}

// https://www.geeksforgeeks.org/zigzag-or-diagonal-traversal-of-matrix/
// https://leetcode.com/problems/diagonal-traverse/solutions/459889/diagonal-traverse/?orderBy=most_relevant
#[allow(dead_code)]
pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let n = mat.len();
    let m = mat[0].len();
    let mut res = vec![];

    // We have to go over all the elements in the first
    // row and the last column to cover all possible diagonals
    for d in 0..n + m - 1 {
        let mut tmp = vec![];
        // We need to figure out the "head" of this diagonal
        // The elements in the first row and the last column
        // are the respective heads.
        let mut r = if d < m { 0 } else { d - m + 1 } as i32;
        let mut c = if d < m { d } else { m - 1 } as i32;
        // Iterate until one of the indices goes out of scope
        // Take note of the index math to go down the diagonal
        while r < n as i32 && c > -1 {
            tmp.push(mat[r as usize][c as usize]);
            r += 1;
            c -= 1;
        }
        // Reverse even numbered diagonals. The
        // article says we have to reverse odd
        // numbered articles but here, the numbering
        // is starting from 0 :P
        if d % 2 == 0 {
            tmp.reverse();
        }
        res.extend(tmp.into_iter());
    }
    res
}

// https://leetcode.com/problems/decode-the-slanted-ciphertext/solutions/1576914/jump-columns-1/
pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
    let encoded_text = encoded_text.chars().collect::<Vec<_>>();
    let mut ans = String::new();
    let cols = encoded_text.len() as i32 / rows;
    for i in 0..cols {
        let mut j = i;
        while j < encoded_text.len() as i32 {
            ans.push(encoded_text[j as usize]);
            j += cols + 1;
        }
    }
    ans.trim_end().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test142() {
        println!(
            "{}",
            min_deletion_size(vec![
                "abc".to_string(),
                "bce".to_string(),
                "cae".to_string()
            ])
        ); // 1
        println!(
            "{}",
            min_deletion_size(vec![
                "zyx".to_string(),
                "wvu".to_string(),
                "tsr".to_string()
            ])
        ); // 3
    }

    #[test]
    fn test143() {
        println!(
            "{:?}",
            find_diagonal_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        ); // [1,2,4,7,5,3,6,8,9]
    }

    #[test]
    fn test144() {
        println!("{}", decode_ciphertext("ch   ie   pr".to_string(), 3));
    }
}
