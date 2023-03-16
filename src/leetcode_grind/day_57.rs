#[allow(dead_code)]
pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    if points.len() == 1 {
        return 1;
    }
    let mut ans = 2;
    for i in 0..points.len() {
        let mut map = HashMap::new();
        for j in 0..points.len() {
            if i != j {
                let atan = (points[j][1] as f64 - points[i][1] as f64)
                    .atan2(points[j][0] as f64 - points[i][0] as f64);
                *map.entry(format!("{}", atan)).or_insert(0) += 1;
            }
        }
        ans = ans.max(*map.values().max().unwrap() + 1);
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test149() {
        println!("{}", max_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]])); // 3
        println!(
            "{}",
            max_points(vec![
                vec![1, 1],
                vec![3, 2],
                vec![5, 3],
                vec![4, 1],
                vec![2, 3],
                vec![1, 4]
            ])
        ); // 4
    }
}
