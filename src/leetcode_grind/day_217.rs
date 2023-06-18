// https://leetcode.com/problems/make-array-strictly-increasing/description/
pub fn make_array_increasing(arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
    pub fn make_array_increasing_top_down(arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        arr2.sort();
        let mut cache = HashMap::new();

        fn bins(arr: &Vec<i32>, t: i32) -> i32 {
            let mut lo = 0;
            let mut hi = arr.len() as i32;
            while lo < hi {
                let mid = (lo + hi) / 2;
                if arr[mid as usize] <= t {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            lo
        }

        fn dfs(
            i: i32,
            prev: i32,
            arr1: &Vec<i32>,
            arr2: &Vec<i32>,
            dp: &mut HashMap<(i32, i32), i32>,
        ) -> i32 {
            if i == arr1.len() as i32 {
                return 0;
            }
            if dp.contains_key(&(i, prev)) {
                return dp[&(i, prev)];
            }
            let mut cost = 2001;
            if arr1[i as usize] > prev {
                cost = dfs(i + 1, arr1[i as usize], arr1, arr2, dp);
            }

            let idx = bins(arr2, prev);

            if idx < arr2.len() as i32 {
                cost = cost.min(1 + dfs(i + 1, arr2[idx as usize], arr1, arr2, dp));
            }

            dp.insert((i, prev), cost);
            cost
        }

        let ans = dfs(0, -1, &arr1, &arr2, &mut cache);
        if ans < 2001 {
            ans
        } else {
            -1
        }
    }

    pub fn make_array_increasing_bottom_up(arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
        fn bins(arr: &Vec<i32>, t: i32) -> i32 {
            let mut lo = 0;
            let mut hi = arr.len() as i32;
            while lo < hi {
                let mid = (lo + hi) / 2;
                if arr[mid as usize] <= t {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            lo
        }

        use std::collections::HashMap;
        arr2.sort();
        let n = arr2.len() as i32;
        let mut dp = HashMap::new();
        dp.insert(-1, 0);

        for i in 0..arr1.len() {
            let mut new_dp = HashMap::new();
            for &prev in dp.keys() {
                if arr1[i] > prev {
                    new_dp.insert(
                        arr1[i],
                        *(new_dp.get(&arr1[i]).unwrap_or(&i32::MAX).min(&dp[&prev])),
                    );
                }
                let idx = bins(&arr2, prev);
                if idx < n {
                    new_dp.insert(
                        arr2[idx as usize],
                        *(new_dp
                            .get(&arr2[idx as usize])
                            .unwrap_or(&i32::MAX)
                            .min(&(1 + dp[&prev]))),
                    );
                }
            }
            dp = new_dp;
        }

        let mut ans = i32::MAX;
        for &v in dp.values() {
            ans = ans.min(v);
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }

    make_array_increasing_bottom_up(arr1, arr2)
}

// https://leetcode.com/problems/minimum-operations-to-make-the-array-increasing/description/
pub fn min_operations(mut nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] + 1 {
            continue;
        }
        let diff = nums[i - 1] + 1 - nums[i];
        nums[i] += diff;
        ans += diff;
    }
    ans
}

// https://leetcode.com/problems/non-decreasing-array/description/
pub fn check_possibility(mut nums: Vec<i32>) -> bool {
    let mut count = 0;
    for i in 1..nums.len() {
        if nums[i - 1] > nums[i] {
            if count == 1 {
                return false;
            }

            count += 1;

            if i < 2 || nums[i - 2] <= nums[i] {
                nums[i - 1] = nums[i];
            } else {
                nums[i] = nums[i - 1];
            }
        }
    }
    true
}

// https://leetcode.com/problems/find-good-days-to-rob-the-bank/description/
pub fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
    let mut p1 = vec![0; security.len()];
    for i in 1..p1.len() {
        if security[i - 1] >= security[i] {
            p1[i] = 1 + p1[i - 1];
        }
    }
    // println!("{:?}", p1);

    let mut p2 = vec![0; security.len()];
    for i in (0..p2.len() - 1).rev() {
        if security[i + 1] >= security[i] {
            p2[i] = 1 + p2[i + 1];
        }
    }
    // println!("{:?}", p2);

    let mut ans = vec![];

    for i in 0..security.len() {
        if p1[i] >= time && p2[i] >= time {
            ans.push(i as i32);
        }
    }

    ans
}

// https://leetcode.com/problems/make-array-non-decreasing-or-non-increasing/
pub fn convert_array(nums: Vec<i32>) -> i32 {
    use std::collections::BinaryHeap;
    fn calc(nums: &Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut res = 0;
        for &num in nums {
            if let Some(&t) = heap.peek() {
                if t >= num {
                    res += (num as i32 - t as i32).abs();
                    heap.pop();
                    heap.push(num);
                }
            }
            heap.push(num);
        }
        res
    }
    let incr = calc(&nums);
    let decr = calc(&nums.into_iter().map(|x| -x).collect());
    // println!("i {} d {}", incr, decr);
    incr.min(decr)
}