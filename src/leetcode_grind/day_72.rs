// https://leetcode.com/problems/find-the-town-judge/
pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let mut in_degree = vec![0; n as usize + 1];
    let mut out_degree = vec![0; n as usize + 1];
    for t in trust {
        in_degree[t[1] as usize] += 1;
        out_degree[t[0] as usize] += 1;
    }
    for i in 1..=n as usize {
        if in_degree[i] == n - 1 && out_degree[i] == 0 {
            return i as i32;
        }
    }
    -1
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test173() {
        println!("{}", find_judge(3, vec![vec![1, 3], vec![2, 3]])); // 3
    }
}
