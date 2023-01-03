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
}
