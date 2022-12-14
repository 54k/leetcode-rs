#[allow(dead_code)]
pub fn rob(nums: Vec<i32>) -> i32 {
    fn all(nums: Vec<i32>) -> i32 {
        fn go(nums: &Vec<i32>, i: i32, seen: &mut Vec<bool>) -> i32 {
            if i < 0 || i > nums.len() as i32 - 1 || seen[i as usize] {
                return 0;
            }

            seen[i as usize] = true;
            if i > 0 {
                seen[i as usize - 1] = true;
            }
            if i < nums.len() as i32 - 1 {
                seen[i as usize + 1] = true;
            }

            let mut max = 0;
            for j in 0..nums.len() as i32 {
                if !seen[j as usize] {
                    max = max.max(go(nums, j, &mut seen.clone()));
                }
            }
            nums[i as usize] + max
        }

        let mut res = 0;
        for i in 0..nums.len() as i32 {
            res = res.max(go(&nums, i, &mut vec![false; nums.len()]));
        }
        res
    }

    fn remember(nums: Vec<i32>) -> i32 {
        const INF: i32 = -10000007;
        fn go(nums: &Vec<i32>, i: i32, seen: &mut Vec<bool>, memo: &mut Vec<i32>) -> i32 {
            if i < 0 || i > nums.len() as i32 - 1 || seen[i as usize] {
                return 0;
            }

            if memo[i as usize] > INF {
                return memo[i as usize];
            }

            seen[i as usize] = true;
            if i > 0 {
                seen[i as usize - 1] = true;
            }
            if i < nums.len() as i32 - 1 {
                seen[i as usize + 1] = true;
            }

            let mut max = 0;
            for j in 0..nums.len() as i32 {
                if !seen[j as usize] {
                    max = max.max(go(nums, j, &mut seen.clone(), memo));
                }
            }

            memo[i as usize] = nums[i as usize] + max;
            memo[i as usize]
        }
        let mut memo = vec![INF; nums.len()];
        let mut res = 0;
        for i in 0..nums.len() as i32 {
            res = res.max(go(&nums, i, &mut vec![false; nums.len()], &mut memo));
        }
        res
    }

    fn turn(nums: Vec<i32>) -> i32 {
        let mut prev = 0;
        let mut last = 0;

        for curr in nums {
            let p = prev;
            prev = last;
            last = last.max(p + curr);
        }

        last
    }

    turn(nums)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test115() {
        println!("{}", rob(vec![2, 1, 1, 2])); // 4
        println!("{}", rob(vec![1, 2, 3, 1])); // 4
        println!("{}", rob(vec![2, 7, 9, 3, 1])); // 12
        println!(
            "{}",
            rob(vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ])
        ); // 0
        println!(
            "{}",
            rob(vec![
                183, 219, 57, 193, 94, 233, 202, 154, 65, 240, 97, 234, 100, 249, 186, 66, 90, 238,
                168, 128, 177, 235, 50, 81, 185, 165, 217, 207, 88, 80, 112, 78, 135, 62, 228, 247,
                211
            ])
        ); // 3365
    }
}
