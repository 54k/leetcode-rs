#[allow(dead_code)]
pub fn jump(nums: Vec<i32>) -> i32 {
    fn greedy(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }

        let mut nums = nums.into_iter().map(|x| x as usize).collect::<Vec<usize>>();
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
pub fn next_greater_elements(mut nums: Vec<i32>) -> Vec<i32> {
    fn brute(mut nums: Vec<i32>) -> Vec<i32> {
        nums.extend(nums.iter().copied().collect::<Vec<i32>>());

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

    fn smart(mut nums: Vec<i32>) -> Vec<i32> {
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
}
