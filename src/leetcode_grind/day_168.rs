// https://leetcode.com/problems/checking-existence-of-edge-length-limited-paths/
pub fn distance_limited_paths_exist(
    n: i32,
    mut edge_list: Vec<Vec<i32>>,
    queries: Vec<Vec<i32>>,
) -> Vec<bool> {
    struct UF {
        group: Vec<usize>,
        rank: Vec<usize>,
    }
    impl UF {
        fn new(n: usize) -> Self {
            Self {
                group: (0..n).into_iter().collect(),
                rank: vec![1; n],
            }
        }
        fn find(&mut self, x: usize) -> usize {
            if self.group[x] != x {
                self.group[x] = self.find(self.group[x]);
            }
            self.group[x]
        }
        fn union(&mut self, x: usize, y: usize) {
            let mut x = self.find(x);
            let mut y = self.find(y);
            if x == y {
                return;
            }
            if self.rank[x] < self.rank[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.group[y] = self.group[x];
            self.rank[x] += self.rank[y];
        }
        fn is_same(&mut self, x: usize, y: usize) -> bool {
            self.find(x) == self.find(y)
        }
    }

    fn sort(arr: &mut Vec<Vec<i32>>) {
        arr.sort_by_key(|x| x[2]);
    }

    let mut query_with_index = vec![];
    for (i, q) in queries.iter().enumerate() {
        query_with_index.push(vec![q[0], q[1], q[2], i as i32]);
    }

    sort(&mut edge_list);
    sort(&mut query_with_index);

    let mut uf = UF::new(n as usize);
    let mut ans = vec![false; queries.len()];

    let mut edge_idx = 0;
    for i in 0..queries.len() {
        let (p, q, d, qi) = (
            query_with_index[i][0],
            query_with_index[i][1],
            query_with_index[i][2],
            query_with_index[i][3],
        );

        while edge_idx < edge_list.len() && edge_list[edge_idx][2] < d {
            uf.union(
                edge_list[edge_idx][0] as usize,
                edge_list[edge_idx][1] as usize,
            );
            edge_idx += 1;
        }

        ans[qi as usize] = uf.is_same(p as usize, q as usize);
    }

    ans
}

// https://leetcode.com/problems/duplicate-zeros/description/
pub fn duplicate_zeros(arr: &mut Vec<i32>) {
    let mut len = arr.len() - 1;
    let mut possible_dups = 0;
    let mut left = 0;

    while left <= len - possible_dups {
        if arr[left] == 0 {
            if left == len - possible_dups {
                arr[len] = 0;
                len -= 1;
                break;
            }
            possible_dups += 1;
        }
        left += 1;
    }

    for i in (0..=len - possible_dups).rev() {
        if arr[i] == 0 {
            arr[i + possible_dups] = 0;
            possible_dups -= 1;
            arr[i + possible_dups] = 0;
        }
        arr[i + possible_dups] = arr[i];
    }
}

pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
    let mut max_so_far = i32::MIN;
    for i in (0..arr.len()).rev() {
        let tmp = arr[i];
        if i < arr.len() - 1 {
            arr[i] = max_so_far;
        } else {
            arr[i] = -1;
        }
        max_so_far = max_so_far.max(tmp);
    }
    arr
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test462() {
        println!(
            "{:?}",
            distance_limited_paths_exist(
                3,
                vec![vec![0, 1, 2], vec![1, 2, 4], vec![2, 0, 8], vec![1, 0, 16]],
                vec![vec![0, 1, 2], vec![0, 2, 5]],
            )
        ); // false, true
    }

    #[test]
    fn test463() {
        let mut v = vec![1, 0, 2, 3, 0, 4, 5, 0];
        duplicate_zeros(&mut v);
        println!("{:?}", v);
    }

    #[test]
    fn test464() {
        println!("{:?}", replace_elements(vec![17, 18, 5, 4, 6, 1])); // [18, 6, 6, 6, 1, -1]
    }
}
