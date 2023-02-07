use std::task::ready;

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

// https://leetcode.com/problems/remove-all-occurrences-of-a-substring/
pub fn remove_occurrences(s: String, part: String) -> String {
    let mut s = s.chars().collect::<Vec<_>>();
    let part = part.chars().collect::<Vec<_>>();
    loop {
        let mut ans = String::new();
        let mut i = 0;
        let len = s.len();
        let mut found = false;
        while i < len {
            if !found {
                let mut j = 0;
                while j < part.len() && i < len && s[i] == part[j] {
                    i += 1;
                    j += 1;
                }
                if j < part.len() {
                    i -= j;
                } else {
                    found = true
                }
            }
            if i < len {
                ans.push(s[i]);
                i += 1;
            }
        }
        s = ans.chars().collect();
        ans.clear();
        if !found {
            break;
        }
    }
    s.into_iter().collect()
}

// https://leetcode.com/problems/valid-palindrome/description/
pub fn is_palindrome(s: String) -> bool {
    let s = s.chars().collect::<Vec<_>>();
    let mut left = 0;
    let mut right = s.len() - 1;

    while left < right {
        if left < right && !char::is_alphanumeric(s[left]) {
            left += 1;
            continue;
        }
        if right > left && !char::is_alphanumeric(s[right]) {
            right -= 1;
            continue;
        }
        if s[left].to_lowercase().to_string() != s[right].to_lowercase().to_string() {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
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

    #[test]
    fn test208() {
        println!(
            "{}",
            remove_occurrences("daabcbaabcbc".to_string(), "abc".to_string())
        ); // "dab"
        println!(
            "{}",
            remove_occurrences("eemckxmckx".to_string(), "emckx".to_string())
        ); // ""
        println!(
            "{}",
            remove_occurrences("axxxxyyyyb".to_string(), "xy".to_string())
        ); // "ab"
        println!("{}", remove_occurrences("a".to_string(), "aa".to_string())); // "a"
        println!(
            "{}",
            remove_occurrences(
                "hhvhvaahvahvhvaavhvaasshvahvaln".to_string(),
                "hva".to_string()
            )
        ); // "ssln"
        println!(
            "{}",
            remove_occurrences("aabababa".to_string(), "aba".to_string())
        ); // "ba"
    }

    #[test]
    fn test209() {
        println!(
            "{}",
            is_palindrome("A man, a plan, a canal: Panama".to_string())
        ); // true
        println!("{}", is_palindrome("a.".to_string())); // false
        println!("{}", is_palindrome("race a car".to_string())); // false
    }
}
