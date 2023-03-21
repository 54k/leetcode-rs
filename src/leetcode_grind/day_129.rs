// https://leetcode.com/problems/number-of-zero-filled-subarrays/description/
// https://leetcode.com/problems/number-of-zero-filled-subarrays/editorial/
pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
    let mut ans = 0;
    let mut current = 0;
    for n in nums {
        if n == 0 {
            current += 1;
        } else {
            current = 0;
        }
        ans += current
    }
    ans
}

// https://leetcode.com/problems/split-array-into-consecutive-subsequences/description/
// https://leetcode.com/problems/split-array-into-consecutive-subsequences/solutions/106496/java-o-n-time-o-n-space/
// https://leetcode.com/problems/split-array-into-consecutive-subsequences/solutions/106495/java-o-n-time-o-1-space-solution-greedily-extending-shorter-subsequence/
pub fn is_possible(nums: Vec<i32>) -> bool {
    // pub fn from_leetcode_rust(nums: Vec<i32>) -> bool {
    //     let mut counter = [0_i32; 2002];
    //
    //     for n in nums.iter() {
    //         counter[(1000 + n) as usize] += 1;
    //     }
    //
    //     for n in nums {
    //         let mut cnt = counter[(1000 + n) as usize];
    //         if cnt == 0 {
    //             continue;
    //         }
    //
    //         let mut count = 0;
    //         let mut a = n;
    //         while 0 < counter[(1000 + a) as usize] {
    //             if counter[(1000 + a) as usize] < cnt {
    //                 break;
    //             }
    //             cnt = counter[(1000 + a) as usize];
    //             counter[(1000 + a) as usize] -= 1;
    //             count += 1;
    //             a += 1;
    //         }
    //         if count < 3 {
    //             return false;
    //         }
    //     }
    //     true
    // }
    fn using_maps(nums: Vec<i32>) -> bool {
        use std::collections::HashMap;
        let mut freq = HashMap::new();
        let mut append_freq = HashMap::new();
        for &num in &nums {
            *freq.entry(num).or_insert(0) += 1;
        }
        for &num in &nums {
            if *freq.get(&num).unwrap() == 0 {
                continue;
            } else if *append_freq.get(&num).unwrap_or(&0) > 0 {
                *append_freq.get_mut(&num).unwrap() -= 1;
                *append_freq.entry(num + 1).or_insert(0) += 1;
            } else if *freq.get(&(num + 1)).unwrap_or(&0) > 0
                && *freq.get(&(num + 2)).unwrap_or(&0) > 0
            {
                *freq.get_mut(&(num + 1)).unwrap() -= 1;
                *freq.get_mut(&(num + 2)).unwrap() -= 1;
                *append_freq.entry(num + 3).or_insert(0) += 1;
            } else {
                return false;
            }
            *freq.get_mut(&num).unwrap() -= 1;
        }
        true
    }
    using_maps(nums)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test360() {
        println!("{}", zero_filled_subarray(vec![1, 3, 0, 0, 2, 0, 0, 4])); // 6
        println!("{}", zero_filled_subarray(vec![0, 0, 0, 2, 0, 0])); // 9
    }

    #[test]
    fn test361() {
        println!("{}", is_possible(vec![1, 2, 3, 3, 4, 5])); // true
        println!("{}", is_possible(vec![1, 2, 3, 3, 4, 4, 5, 5])); // true
        println!("{}", is_possible(vec![1, 2, 3, 4, 4, 5])); // false
    }
}
