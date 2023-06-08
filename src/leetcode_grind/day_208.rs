// https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/description/
pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid[0].len();
    let mut count = 0;
    let mut row_neg_idx = n as i32 - 1;

    for row in grid {
        while row_neg_idx >= 0 && row[row_neg_idx as usize] < 0 {
            row_neg_idx -= 1;
        }
        count += (n as i32 - row_neg_idx - 1) as i32;
    }

    count
}

// https://leetcode.com/discuss/interview-question/algorithms/125398/given-a-stack-sort-it-in-non-decreasing-order
pub fn sort_stack(mut s: Vec<i32>) -> Vec<i32> {
    let mut t = vec![];
    while let Some(top) = s.pop() {
        while !t.is_empty() && *t.last().unwrap() < top {
            s.push(t.pop().unwrap());
        }
        t.push(top);
    }
    while let Some(top) = t.pop() {
        s.push(top);
    }
    s
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test505() {
        println!("{:?}", sort_stack(vec![4, 1, 3, 2, 5, 0, 3]));
    }
}
