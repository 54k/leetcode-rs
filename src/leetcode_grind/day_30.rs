#[allow(dead_code)]
pub fn jump(nums: Vec<i32>) -> i32 {
    fn greedy(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }

        let nums = nums.into_iter().map(|x| x as usize).collect::<Vec<usize>>();
        let mut jumps = 0;
        let mut i = 0;

        while i < nums.len() - 1 {
            let mut max = 0;
            let mut next_first = i + 1;
            let next_last = i + nums[i];

            if next_last >= nums.len() - 1 {
                return jumps + 1;
            }

            while next_first <= next_last {
                if max <= nums[next_first] + next_first {
                    max = nums[next_first] + next_first;
                    i = next_first;
                }
                next_first += 1;
            }
            jumps += 1;
        }

        jumps
    }

    greedy(nums)
}

#[allow(dead_code)]
pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    fn brute(mut nums: Vec<i32>) -> Vec<i32> {
        nums.extend(nums.to_vec());

        let mut result = vec![];
        let mut stack = vec![];

        for i in 0..nums.len() / 2 {
            let e = nums[i];
            for j in i + 1..i + 1 + nums.len() / 2 {
                let e2 = nums[j];
                stack.push(e2);
                if e2 > e {
                    break;
                }
            }
            while !stack.is_empty() && *stack.last().unwrap() <= e {
                stack.pop();
            }
            result.push(*stack.last().unwrap_or(&-1));
        }

        result
    }

    fn smart(nums: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut result = vec![-1; nums.len()];
        for i in (0..nums.len() * 2 - 1).rev() {
            let e = nums[i % nums.len()];
            while !stack.is_empty() && nums[*stack.last().unwrap()] <= e {
                stack.pop();
            }
            if stack.is_empty() {
                result[i % nums.len()] = -1;
            } else {
                result[i % nums.len()] = nums[*stack.last().unwrap()];
            }
            stack.push(i % nums.len());
        }
        result
    }

    smart(nums)
}

#[allow(dead_code)]
pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    fn dfs(grid: &mut Vec<Vec<char>>, i: i32, j: i32) -> bool {
        if i < 0 || i >= grid.len() as i32 || j < 0 || j >= grid[0].len() as i32 {
            return false;
        }
        if grid[i as usize][j as usize] == '0' {
            return false;
        }

        grid[i as usize][j as usize] = '0';
        dfs(grid, i + 1, j);
        dfs(grid, i - 1, j);
        dfs(grid, i, j + 1);
        dfs(grid, i, j - 1);

        true
    }

    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if dfs(&mut grid, i as i32, j as i32) {
                ans += 1;
            }
        }
    }

    ans
}

#[allow(dead_code)]
pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    intervals.push(new_interval);
    intervals.sort_by(|i, j| i[0].cmp(&j[0]));

    let mut start = intervals[0][0];
    let mut end = intervals[0][1];

    for i in 1..intervals.len() {
        let interval = &intervals[i];

        if end >= interval[0] {
            end = end.max(interval[1]);
        } else {
            res.push(vec![start, end]);
            start = interval[0];
            end = interval[1];
        }
    }
    res.push(vec![start, end]);
    res
}

#[allow(dead_code)]
pub fn interval_intersection(
    first_list: Vec<Vec<i32>>,
    second_list: Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let mut i = 0;
    let mut j = 0;

    while i < first_list.len() && j < second_list.len() {
        let lo = first_list[i][0].max(second_list[j][0]);
        let hi = first_list[i][1].min(second_list[j][1]);

        if lo <= hi {
            ans.push(vec![lo, hi]);
        }

        if first_list[i][1] < second_list[j][1] {
            i += 1;
        } else {
            j += 1;
        }
    }

    ans
}

#[allow(dead_code)]
pub fn match_players_and_trainers(mut players: Vec<i32>, mut trainers: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut i = 0;
    let mut j = 0;

    players.sort();
    trainers.sort();

    while i < players.len() && j < trainers.len() {
        if players[i] <= trainers[j] {
            ans += 1;
            i += 1;
        }
        j += 1;
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test105() {
        println!("{}", jump(vec![2, 3, 1, 1, 4]));
        println!("{}", jump(vec![2, 3, 0, 1, 4]));
        println!("{}", jump(vec![1]));
        println!("{}", jump(vec![2, 0]));
        println!("{}", jump(vec![1, 2, 3]));
        println!("{}", jump(vec![1, 1, 1, 1]));
        println!("{}", jump(vec![1, 2, 1, 1, 1]));
        println!("{}", jump(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 0]));
    }

    #[test]
    fn test106() {
        println!("{:?}", next_greater_elements(vec![1, 2, 3, 4, 3])); // [2,3,4,-1,4]
        println!("{:?}", next_greater_elements(vec![1, 2, 1])); // [2,-1,2]
        println!("{:?}", next_greater_elements(vec![1])); // [-1]
    }

    #[test]
    fn test107() {
        println!(
            "{}",
            num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0'],
            ])
        ); // 1
    }

    #[test]
    fn test108() {
        println!("{:?}", insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]));
    }

    #[test]
    fn test109() {
        println!(
            "{:?}",
            interval_intersection(
                vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]],
                vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]]
            )
        ); // [[1,2],[5,5],[8,10],[15,23],[24,24],[25,25]]
    }

    #[test]
    fn test110() {
        println!(
            "{:?}",
            match_players_and_trainers(vec![4, 7, 9], vec![8, 2, 5, 8])
        ); // 2
        println!("{:?}", match_players_and_trainers(vec![1, 1, 1], vec![10])); // 1
        println!(
            "{:?}",
            match_players_and_trainers(vec![2, 2, 1], vec![10, 1])
        ); // 2
    }
}
