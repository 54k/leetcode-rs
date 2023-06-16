// https://leetcode.com/problems/number-of-ways-to-reorder-array-to-get-same-bst/description/
pub fn num_of_ways(nums: Vec<i32>) -> i32 {
    // For those that had trouble understanding the editorial, here's an alternative explanation.

    // The problem boils down to figuring out the count of acceptable item orders that recreate the left tree and the right tree,
    // then combining those two solutions to obtain the count of orders for the whole tree.
    // If you have L orderings on the left and R orderings on the right, the total number of orderings is L * R * C,
    // where C is the number of ways in which the left and right arrays can be Чередованием.
    // If you have ls elements on the left, and rs elements on the right,
    // that's comb(ls + rs, ls) (or, equivalently, comb(ls + rs, rs)) – that is,
    // the number of ways in which you can select the positions for the ls items in the resulting array of size ls + rs.

    // Example 2 is good for thinking through this. At the root,
    // both the left and right trees have one possible ordering.
    // For the whole tree, we just need to figure out the number of ways in which both arrays,
    // of size 2, can be interleaved, which is comb(4, 2) = 6. The final solution is 1*1*6 - 1 = 5 (subtracting 1 since the problem asks for reorderings).
    const MOD: i64 = 1_000_000_007;
    fn dfs(nums: &Vec<i32>, table: &Vec<Vec<i64>>) -> i64 {
        let m = nums.len();
        if m < 3 {
            return 1;
        }

        let mut left_nodes = vec![];
        let mut right_nodes = vec![];

        for i in 1..m {
            if nums[i] < nums[0] {
                left_nodes.push(nums[i]);
            } else {
                right_nodes.push(nums[i]);
            }
        }

        let left_ways = dfs(&left_nodes, table) % MOD;
        let right_ways = dfs(&right_nodes, table) % MOD;

        ((left_ways * right_ways) % MOD) * table[m - 1][left_nodes.len()] % MOD
    }

    let m = nums.len();
    let mut table = vec![vec![0; m]; m];
    for i in 0..m {
        table[i][i] = 1;
        table[i][0] = table[i][i];
    }
    for i in 2..m {
        for j in 1..i {
            table[i][j] = (table[i - 1][j - 1] + table[i - 1][j]) % MOD;
        }
    }

    ((dfs(&nums, &table) - 1) % MOD) as i32
}

// https://leetcode.com/problems/the-number-of-weak-characters-in-the-game/description/
pub fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
    pub fn using_sort(mut properties: Vec<Vec<i32>>) -> i32 {
        properties.sort_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });

        let mut weak_chars = 0;
        let mut max_def = 0;

        for i in (0..properties.len()).rev() {
            if properties[i][1] < max_def {
                weak_chars += 1;
            }
            max_def = max_def.max(properties[i][1])
        }

        weak_chars
    }
    pub fn using_greedy_approach(properties: Vec<Vec<i32>>) -> i32 {
        // Find the maximum atack value
        let mut max_attack = 0;
        for prop in &properties {
            let attack = prop[0];
            max_attack = max_attack.max(attack);
        }

        // Store the maximum defense for an attack value
        let mut max_def = vec![0; max_attack as usize + 2];
        for prop in &properties {
            let attack = prop[0];
            let def = prop[1];

            max_def[attack as usize] = max_def[attack as usize].max(def);
        }

        // Store the maximum defense for attack greater than or equal to a value
        for i in (0..max_attack).rev() {
            max_def[i as usize] = max_def[i as usize].max(max_def[i as usize + 1]);
        }

        let mut weak_chars = 0;
        for prop in &properties {
            let (atk, def) = (prop[0], prop[1]);

            if def < max_def[atk as usize + 1] {
                weak_chars += 1;
            }
        }
        weak_chars
    }
    using_greedy_approach(properties)
}

// https://leetcode.com/problems/minimum-moves-to-equal-array-elements-ii/description/
pub fn min_moves2(nums: Vec<i32>) -> i32 {
    pub fn brute_force_tle(nums: Vec<i32>) -> i32 {
        let mut ans = i64::MAX;
        let (mut min, mut max) = (i32::MAX, i32::MIN);
        for &num in &nums {
            min = min.min(num);
            max = max.max(num);
        }

        for i in min..=max {
            let mut sum = 0;
            for &num in &nums {
                sum += (num - i).abs();
            }
            ans = ans.min(sum as i64);
        }
        ans as i32
    }
    pub fn sort_approach(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut min = i64::MAX;
        let mut sum = 0;
        let mut total = 0;
        for &num in &nums {
            total += num;
        }
        for i in 0..nums.len() {
            let ans = (nums[i] as i64 * i as i64 - sum as i64)
                + ((total as i64 - sum as i64) - nums[i] as i64 * (nums.len() as i64 - i as i64));
            min = min.min(ans);
            sum += nums[i];
        }
        min as i32
    }
    todo!()
}
