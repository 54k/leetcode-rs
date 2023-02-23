// https://leetcode.com/problems/ipo/description/
// https://leetcode.com/problems/ipo/solutions/2959870/ipo/
pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    use std::collections::*;
    let mut heap = BinaryHeap::new();
    let mut sorted_projects = vec![];
    for i in 0..profits.len() {
        sorted_projects.push((capital[i], profits[i]));
    }
    sorted_projects.sort();

    let mut ptr = 0;
    for _ in 0..k {
        while ptr < sorted_projects.len() && sorted_projects[ptr].0 <= w {
            heap.push(sorted_projects[ptr].1);
            ptr += 1;
        }
        if heap.is_empty() {
            break;
        }
        w += heap.pop().unwrap();
    }
    w
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test276() {
        println!(
            "{}",
            find_maximized_capital(2, 0, vec![1, 2, 3], vec![1, 2, 3])
        ); // 4
    }
}
