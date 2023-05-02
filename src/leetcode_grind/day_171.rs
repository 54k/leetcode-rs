use std::sync::Arc;

// https://leetcode.com/problems/sign-of-the-product-of-an-array/
pub fn array_sign(nums: Vec<i32>) -> i32 {
    let mut min = 0;
    for num in nums {
        if num < 0 {
            min += 1;
        } else if num == 0 {
            return 0;
        }
    }
    if min % 2 == 1 {
        -1
    } else {
        1
    }
}

// https://leetcode.com/problems/walls-and-gates/description/
pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
    use std::collections::VecDeque;
    const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut q = VecDeque::new();
    for i in 0..rooms.len() {
        for j in 0..rooms[i].len() {
            if rooms[i][j] == 0 {
                q.push_back((i as i32, j as i32));
            }
        }
    }

    while let Some((i, j)) = q.pop_front() {
        for dir in DIRS {
            let (x, y) = (i as i32 + dir.0, j as i32 + dir.1);
            if x < 0
                || x >= rooms.len() as i32
                || y < 0
                || y >= rooms[0].len() as i32
                || rooms[x as usize][y as usize] != i32::MAX
            {
                continue;
            }
            rooms[x as usize][y as usize] = rooms[i as usize][j as usize] + 1;
            q.push_back((x, y));
        }
    }
}

// https://leetcode.com/problems/design-an-atm-machine/
struct ATM {
    store: [(i32, i64); 5],
}

impl ATM {
    fn new() -> Self {
        Self {
            store: [(500, 0), (200, 0), (100, 0), (50, 0), (20, 0)],
        }
    }

    fn deposit(&mut self, banknotes_count: Vec<i32>) {
        for (i, cnt) in banknotes_count.into_iter().rev().enumerate() {
            self.store[i].1 += cnt as i64;
        }
    }

    fn withdraw(&mut self, mut amount: i32) -> Vec<i32> {
        let mut ans = vec![0; 5];
        let mut store = self.store.clone();

        for (i, (amt, amt_cnt)) in store.iter_mut().enumerate() {
            if *amt > amount || *amt_cnt == 0 {
                continue;
            }

            let cnt = (amount as i64 / *amt as i64).min(*amt_cnt);
            *amt_cnt -= cnt;

            ans[5 - i - 1] = cnt as i32;
            amount -= cnt as i32 * *amt;
        }

        if amount > 0 {
            return vec![-1];
        }
        self.store = store;
        ans
    }
}

// https://leetcode.com/problems/target-sum/description/
pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    fn dfs(nums: &Vec<i32>, target: i32, i: usize, ans: &mut i32) {
        if i > nums.len() - 1 {
            return;
        }

        if target == 0 {
            *ans += 1;
            return;
        }
        
        dfs(nums, target + nums[i], i + 1, ans);
        dfs(nums, target - nums[i], i + 1, ans);
    }
    let mut ans = 0;
    dfs(&nums, target, 0, &mut ans);
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test474() {
        println!("{}", array_sign(vec![-1, 1, -1, 1, -1]));
    }

    #[test]
    fn test475() {
        let mut rooms = vec![
            vec![2147483647, -1, 0, 2147483647],
            vec![2147483647, 2147483647, 2147483647, -1],
            vec![2147483647, -1, 2147483647, -1],
            vec![0, -1, 2147483647, 2147483647],
        ];
        walls_and_gates(&mut rooms);
        println!("{:?}", rooms);
    }

    #[test]
    fn test476() {
        let mut atm = ATM::new();
        atm.deposit(vec![0, 0, 1, 2, 1]);
        println!("{:?}", atm.withdraw(600));
        atm.deposit(vec![0, 1, 0, 1, 1]);
        println!("{:?}", atm.withdraw(600));
        println!("{:?}", atm.withdraw(550));

        let mut atm = ATM::new();
        atm.deposit(vec![0, 10, 0, 3, 0]);
        println!("{:?}", atm.withdraw(500)); // [0,2,0,2,0]
    }

    #[test]
    fn test478() {
        println!("{}", find_target_sum_ways(vec![1, 1, 1, 1, 1], 3)); // 5
    }
}
