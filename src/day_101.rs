// https://leetcode.com/problems/single-element-in-a-sorted-array/description/
pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    fn linear(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for x in nums {
            ans ^= x;
        }
        ans
    }
    fn log_n(nums: Vec<i32>) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len() - 1;
        while lo < hi {
            let mid = (lo + hi) / 2;
            /* if even then check next, if odd then check previous*/
            if nums[mid] == nums[mid ^ 1] {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        nums[lo]
    }
    log_n(nums)
}

// https://leetcode.com/problems/queens-that-can-attack-the-king/description/
pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
    const DIR: [(i32, i32); 8] = [
        (-1, 0),
        (-1, 1),
        (-1, -1),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, 0),
        (1, -1),
    ];
    fn dfs(
        board: &Vec<Vec<i32>>,
        position: (i32, i32),
        direction: (i32, i32),
        result: &mut Vec<Vec<i32>>,
    ) {
        let (x, y) = position;
        if x < 0 || y < 0 || x >= board.len() as i32 || y >= board[0].len() as i32 {
            return;
        }
        if board[x as usize][y as usize] == 1 {
            result.push(vec![x, y]);
            return;
        }
        let next_position = (x + direction.0, y + direction.1);
        dfs(board, next_position, direction, result);
    }
    let mut board = vec![vec![0; 8]; 8];
    for q in queens {
        let x = q[0];
        let y = q[1];
        board[x as usize][y as usize] = 1;
    }
    let mut result = vec![];
    for d in DIR {
        dfs(&board, (king[0], king[1]), d, &mut result);
    }
    result
}

// https://leetcode.com/problems/reverse-string-ii/description/
// https://leetcode.com/problems/reverse-string-ii/solutions/127489/reverse-string-ii/
pub fn reverse_str(s: String, k: i32) -> String {
    let k = k as usize;
    let mut s = s.chars().collect::<Vec<_>>();
    for start in (0..s.len()).step_by(2 * k) {
        let mut i = start;
        let mut j = (start + k - 1).min(s.len() - 1);
        while i < j {
            s.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
    s.into_iter().collect()
}

// https://leetcode.com/problems/longest-consecutive-sequence/description/
// https://leetcode.com/problems/longest-consecutive-sequence/solutions/127576/longest-consecutive-sequence/
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    fn dsu(nums: Vec<i32>) -> i32 {
        use std::collections::*;
        if nums.is_empty() {
            return 0;
        }
        let mut idx_map = HashMap::new();
        let mut uf_size = vec![0; nums.len()];
        let mut uf_array = vec![0; nums.len()];
        for (idx, num) in nums.into_iter().enumerate() {
            uf_array[idx] = idx;
            uf_size[idx] = 1;
            idx_map.insert(num, idx);
        }
        fn find(uf_array: &mut Vec<usize>, x: usize) -> usize {
            if uf_array[x] == x {
                return x;
            }
            uf_array[x] = find(uf_array, uf_array[x]);
            uf_array[x]
        }
        fn union(uf_array: &mut Vec<usize>, uf_size: &mut [i32], x: usize, y: usize) {
            let mut x = find(uf_array, x);
            let mut y = find(uf_array, y);
            if uf_size[x] > uf_size[y] {
                std::mem::swap(&mut x, &mut y);
            }
            uf_array[x] = y;
            uf_size[y] += uf_size[x];
        }
        for (&k, &a) in &idx_map {
            if let Some(&b) = idx_map.get(&(k - 1)) {
                union(&mut uf_array, &mut uf_size, a, b);
            }
        }
        uf_size.into_iter().max().unwrap()
    }
    fn with_set(nums: Vec<i32>) -> i32 {
        use std::collections::*;
        let mut set = HashSet::new();
        for num in nums {
            set.insert(num);
        }
        set.iter()
            .filter(|&&x| !set.contains(&(x - 1)))
            .map(|&x| (x..).take_while(|&x| set.contains(&x)).count())
            .max()
            .unwrap_or(0) as i32
    }
    fn leetcode(nums: Vec<i32>) -> i32 {
        use std::collections::*;
        let mut set = HashSet::new();
        for num in nums {
            set.insert(num);
        }
        let mut longest_streak = 0;
        for &num in set.iter() {
            if !set.contains(&(num - 1)) {
                let mut current_num = num;
                let mut current_streak = 1;

                while set.contains(&(current_num + 1)) {
                    current_num += 1;
                    current_streak += 1;
                }
                longest_streak = longest_streak.max(current_streak);
            }
        }
        longest_streak
    }
    leetcode(nums)
}

// https://leetcode.com/problems/find-three-consecutive-integers-that-sum-to-a-given-number/description/
pub fn sum_of_three(num: i64) -> Vec<i64> {
    todo!()
}

// https://leetcode.com/problems/number-of-ways-to-buy-pens-and-pencils/
pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test261() {
        println!("{}", single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8])); // 2
        println!("{}", single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11])); // 10
    }

    #[test]
    fn test262() {
        println!(
            "{:?}",
            queens_attackthe_king(
                vec![
                    vec![0, 1],
                    vec![1, 0],
                    vec![4, 0],
                    vec![0, 4],
                    vec![3, 3],
                    vec![2, 4],
                ],
                vec![0, 0],
            )
        ); // [[0,1],[1,0],[3,3]]
    }

    #[test]
    fn test263() {
        println!("{}", reverse_str("abcdefg".to_string(), 2)); // bacdfeg
    }

    #[test]
    fn test264() {
        println!("{}", longest_consecutive(vec![100, 4, 200, 1, 3, 2])); // 4
        println!(
            "{}",
            longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1])
        ); // 9
    }

    #[test]
    fn test265() {
        println!("{:?}", sum_of_three(33)); // [10,11,12]
    }

    #[test]
    fn test266() {
        println!("{}", ways_to_buy_pens_pencils(20, 10, 5)); // 9
        println!("{}", ways_to_buy_pens_pencils(5, 10, 10)); // 1
    }
}
