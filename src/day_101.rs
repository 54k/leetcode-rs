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
    if num % 3 == 0 {
        vec![(num / 3) - 1, num / 3, (num / 3) + 1]
    } else {
        vec![]
    }
}

// https://leetcode.com/problems/number-of-ways-to-buy-pens-and-pencils/
pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
    todo!()
}

// https://leetcode.com/problems/maximum-consecutive-floors-without-special-floors/description/
// Say there are n special floors. After sorting special,
// we have answer = max(answer, special[i] – special[i – 1] – 1) for all 0 < i ≤ n.
pub fn max_consecutive(bottom: i32, top: i32, mut special: Vec<i32>) -> i32 {
    fn solution1(bottom: i32, top: i32, mut special: Vec<i32>) -> i32 {
        let mut ans = 0;
        special.sort();
        let mut start = bottom;
        if special[0] - start > 0 {
            ans = special[0] - start;
            start = special[0];
        }
        for end in special.into_iter().skip(1) {
            if end > start {
                ans = ans.max(end - start - 1);
            }
            start = end;
        }
        if top > start {
            ans = ans.max(top - start);
        }
        ans
    }
    fn solution2(mut bottom: i32, top: i32, mut special: Vec<i32>) -> i32 {
        special.sort();
        let mut out = 0;
        for s in special {
            out = out.max(s - bottom);
            bottom = s + 1;
        }
        out.max(top - bottom + 1)
    }
    solution2(bottom, top, special)
}

// https://leetcode.com/problems/largest-combination-with-bitwise-and-greater-than-zero/description/
// Intuition: we need to count integers that share the same bit. Bitwise AND of those integers will be positive.
// We count integers that share each bit, and return the maximum.
// https://leetcode.com/problems/largest-combination-with-bitwise-and-greater-than-zero/solutions/2039717/check-each-bit/
pub fn largest_combination(candidates: Vec<i32>) -> i32 {
    let mut max = 0;
    for i in 0..30 {
        let mut count = 0;
        for &c in &candidates {
            if (c >> i) & 1 == 1 {
                count += 1;
            }
        }
        max = max.max(count);
    }
    max
}

// https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/description/
// https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/solutions/1525309/java-c-python-dp-solution/
// Intuition
// Similar to knapsack problem,
// but use bitwise-or sum instead of math sum.
//
// Explanation
// dp[sum] means the number of subsets with bitwise-or sum.
pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
    fn recursive(nums: Vec<i32>) -> i32 {
        fn dfs(nums: &[i32], i: usize, subset_or: i32, max: &mut i32, count_max: &mut i32) {
            if i == nums.len() {
                if subset_or > *max {
                    *max = subset_or;
                    *count_max = 0;
                    *count_max += 1;
                } else if subset_or == *max {
                    *count_max += 1;
                }
                return;
            }
            dfs(nums, i + 1, subset_or | nums[i], max, count_max);
            dfs(nums, i + 1, subset_or, max, count_max);
        }
        let mut max = 0;
        let mut count_max = 0;
        dfs(&nums, 0, 0, &mut max, &mut count_max);
        count_max
    }

    fn knapsack_dp(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut dp = vec![0; 1 << 17];
        dp[0] = 1;
        for num in nums {
            let num = num as usize;
            for i in (0..=max).rev() {
                dp[i | num] += dp[i]
            }
            max |= num;
        }
        dp[max]
    }

    knapsack_dp(nums)
}

// https://leetcode.com/problems/longest-subarray-with-maximum-bitwise-and/description/
pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let max = nums.iter().copied().max().unwrap();
    let mut ans = 0;
    let mut count_longest_max = 0;
    for i in 0..nums.len() {
        if nums[i] == max {
            count_longest_max += 1;
            ans = ans.max(count_longest_max);
        } else {
            count_longest_max = 0;
        }
    }
    ans
}

// https://leetcode.com/problems/count-integers-in-intervals/solutions/2039706/merge-intervals/
// https://leetcode.com/problems/count-integers-in-intervals/description/
struct CountIntervals {}

impl CountIntervals {
    fn new() -> Self {
        Self {}
    }

    fn add(&self, left: i32, right: i32) {}

    fn count(&self) -> i32 {
        0
    }
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
        println!("{:?}", sum_of_three(4)); //
        println!("{:?}", sum_of_three(129)); //
        println!("{:?}", sum_of_three(99)); //
        println!("{:?}", sum_of_three(992)); //
    }

    #[test]
    fn test266() {
        println!("{}", ways_to_buy_pens_pencils(20, 10, 5)); // 9
        println!("{}", ways_to_buy_pens_pencils(5, 10, 10)); // 1
    }

    #[test]
    fn test267() {
        println!("{}", max_consecutive(2, 9, vec![4, 6])); // 3
        println!("{}", max_consecutive(6, 8, vec![7, 6, 8])); // 0
        println!("{}", max_consecutive(28, 50, vec![35, 48])); // 0
    }

    #[test]
    fn test268() {
        println!("{}", largest_combination(vec![16, 17, 71, 62, 12, 24, 14])); // 4
    }

    #[test]
    fn test269() {
        println!("{}", count_max_or_subsets(vec![3, 2, 1, 5])); // 6
        println!("{}", count_max_or_subsets(vec![2, 2, 2])); // 7
    }

    #[test]
    fn test270() {
        println!("{}", longest_subarray(vec![1, 2, 3, 3, 2, 2])); // 2
    }
}
