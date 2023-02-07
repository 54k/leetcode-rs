// https://leetcode.com/problems/fruit-into-baskets/solutions/2960000/fruit-into-baskets/
// https://leetcode.com/problems/fruit-into-baskets/solutions/?orderBy=most_relevant
// Given an array of integers, find the longest subarray that contains at most 2 unique integers.
// (We will call such subarray a valid subarray for convenience)
pub fn total_fruit(fruits: Vec<i32>) -> i32 {
    use std::collections::*;
    fn brute(fruits: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..fruits.len() {
            for j in i..fruits.len() {
                let mut set = HashSet::new();
                for k in i..=j {
                    set.insert(fruits[k]);
                }
                if set.len() <= 2 {
                    ans = ans.max(j - i + 1);
                }
            }
        }
        ans as i32
    }
    fn brute_optimized(fruits: Vec<i32>) -> i32 {
        let mut ans = 0;
        for left in 0..fruits.len() {
            let mut right = left;
            let mut set = HashSet::new();
            while right < fruits.len() {
                if !set.contains(&fruits[right]) && set.len() == 2 {
                    break;
                }
                set.insert(fruits[right]);
                right += 1;
            }
            ans = ans.max(right - left);
        }
        ans as i32
    }
    fn sliding_window(fruits: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut map = HashMap::new();
        while right < fruits.len() {
            *map.entry(fruits[right]).or_insert(0) += 1;
            right += 1;
            if map.len() > 2 {
                let basket = map.get_mut(&fruits[left]).unwrap();
                *basket -= 1;
                if *basket == 0 {
                    map.remove(&fruits[left]);
                }
                left += 1;
            }
        }
        (right - left) as i32
    }
    fn sliding_window2(fruits: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut map = HashMap::new();
        for right in 0..fruits.len() {
            *map.entry(fruits[right]).or_insert(0) += 1;
            while map.len() > 2 {
                let basket = map.get_mut(&fruits[left]).unwrap();
                *basket -= 1;
                if *basket == 0 {
                    map.remove(&fruits[left]);
                }
                left += 1;
            }
            ans = ans.max(right - left + 1)
        }
        ans as i32
    }
    sliding_window(fruits)
}

// https://leetcode.com/problems/sort-array-by-parity/description/
pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
    fn approach1(mut nums: Vec<i32>) -> Vec<i32> {
        // slow fast
        let mut left = 0;
        let mut right = 0;
        while right < nums.len() {
            if nums[right] & 1 == 0 {
                nums.swap(left, right);
                left += 1;
            }
            right += 1;
        }
        nums
    }
    fn approach2(mut nums: Vec<i32>) -> Vec<i32> {
        // meet in the middle?
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            if nums[left] & 1 > nums[right] & 1 {
                nums.swap(left, right);
            }

            if nums[left] & 1 == 0 {
                left += 1;
            }
            if nums[right] & 1 == 1 {
                right -= 1;
            }
        }
        nums
    }
    approach2(nums)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test206() {
        println!("{}", total_fruit(vec![1, 2, 1])); // 3 -> [1,2,1]
        println!("{}", total_fruit(vec![0, 1, 2, 2])); // 3 -> [1,2,2]
        println!("{}", total_fruit(vec![1, 2, 3, 2, 2])); // 4 -> [2,3,2,2]
        println!("{}", total_fruit(vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4])); // 5
    }

    #[test]
    fn test207() {
        println!("{:?}", sort_array_by_parity(vec![3, 1, 2, 4])); // [2,4,3,1]
        println!("{:?}", sort_array_by_parity(vec![0, 2])); // [0, 2]
    }
}
