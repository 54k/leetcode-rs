use std::sync::atomic::Ordering;

// https://leetcode.com/problems/permutation-in-string/description/
pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }
    let mut h1 = vec![0; 26];
    for c in s1.chars() {
        h1[c as usize - 'a' as usize] += 1;
    }
    for i in 0..=(s2.len() - s1.len()) {
        let mut h2 = vec![0; 26];
        let s2 = s2.chars().collect::<Vec<_>>();
        for j in 0..s1.len() {
            h2[s2[i + j] as usize - 'a' as usize] += 1;
        }
        if h1 == h2 {
            return true;
        }
    }
    false
}

// https://leetcode.com/problems/find-k-closest-elements/solutions/106419/o-log-n-java-1-line-o-log-n-k-ruby/ StefanPochmann is a god
// https://leetcode.com/problems/find-k-closest-elements/
pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let mut lo = 0;
    let mut hi = arr.len() - k as usize;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if x - arr[mid] > arr[mid + k as usize] - x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    arr[lo..lo + k as usize].to_vec()
}

// https://leetcode.com/problems/is-subsequence/
pub fn is_subsequence(s: String, t: String) -> bool {
    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();
    let mut i = 0;
    let mut j = 0;
    while i < s.len() && j < t.len() {
        if s[i] != t[j] {
            j += 1;
        } else {
            i += 1;
            j += 1;
        }
    }
    i == s.len()
}

// https://leetcode.com/problems/move-zeroes/
pub fn move_zeroes(nums: &mut Vec<i32>) {
    fn no_std(nums: &mut Vec<i32>) {
        let mut slow = 0;
        let mut fast = 0;
        while fast < nums.len() {
            if nums[fast] != 0 {
                nums.swap(slow, fast);
                slow += 1;
                fast += 1;
            } else {
                fast += 1;
            }
        }
    }
    fn with_std(nums: &mut Vec<i32>) {
        use std::cmp::Ordering;
        nums.sort_by(|a, b| {
            if *a == 0 {
                Ordering::Greater
            } else if *b == 0 {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
    }
    no_std(nums)
}

// https://leetcode.com/problems/adding-spaces-to-a-string/description/
pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
    let mut ans = String::new();
    let s = s.chars().collect::<Vec<_>>();
    let mut i = 0; // string iter
    let mut j = 0; // spaces iter
    while j < spaces.len() {
        if i < spaces[j] {
            ans.push(s[i as usize]);
            i += 1;
        } else {
            ans.push(' ');
            j += 1;
        }
    }
    while i < s.len() as i32 {
        ans.push(s[i as usize]);
        i += 1;
    }
    ans
}

// Using a hashmap, we can map the values of arr2 to their position in arr2.
// After, we can use a custom sorting function.
pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    let map = arr2
        .into_iter()
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect::<HashMap<i32, usize>>();
    arr1.sort_by(|a, b| {
        if map.contains_key(a) && map.contains_key(b) {
            map.get(a).unwrap().cmp(map.get(b).unwrap())
        } else if map.contains_key(a) {
            Ordering::Less
        } else if map.contains_key(b) {
            Ordering::Greater
        } else {
            a.cmp(b)
        }
    });
    arr1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test195() {
        println!(
            "{}",
            check_inclusion("ab".to_string(), "eidbaooo".to_string())
        ); // true
        println!(
            "{}",
            check_inclusion("ab".to_string(), "eidboaoo".to_string())
        ); // false

        println!(
            "{}",
            check_inclusion("ab".to_string(), "eidboaoo".to_string())
        ); // false
    }

    #[test]
    fn test196() {
        println!("{:?}", find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3)); // [1,2,3,4]
        println!("{:?}", find_closest_elements(vec![1, 2, 3, 4, 5], 4, -1)); // [1,2,3,4]
        println!(
            "{:?}",
            find_closest_elements(vec![1, 3, 5, 10, 15, 22, 23, 24, 25], 5, 8)
        ); // [1, 3, 5, 10, 15]
    }

    #[test]
    fn test197() {
        println!(
            "{}",
            is_subsequence("abc".to_string(), "ahbgdc".to_string())
        ); // true
        println!(
            "{}",
            is_subsequence("axc".to_string(), "ahbgdc".to_string())
        ); // false
        println!("{}", is_subsequence("".to_string(), "ahbgdc".to_string())); // false
    }

    #[test]
    fn test198() {
        let mut vec1 = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut vec1);
        println!("{:?}", vec1);
        let mut vec2 = vec![0];
        move_zeroes(&mut vec2);
        println!("{:?}", vec2);
        let mut vec3 = vec![3, 1, 2, 0, 0];
        move_zeroes(&mut vec3);
        println!("{:?}", vec3);
    }

    #[test]
    fn test199() {
        println!(
            "{}",
            add_spaces("LeetcodeHelpsMeLearn".to_string(), vec![8, 13, 15])
        ); // "Leetcode Helps Me Learn"
        println!(
            "{}",
            add_spaces("spacing".to_string(), vec![0, 1, 2, 3, 4, 5, 6])
        ); // " s p a c i n g"
    }

    #[test]
    fn test200() {
        println!(
            "{:?}",
            relative_sort_array(
                vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
                vec![2, 1, 4, 3, 9, 6]
            )
        ); // [2,2,2,1,4,3,3,9,6,7,19]
        println!(
            "{:?}",
            relative_sort_array(vec![28, 6, 22, 8, 44, 17], vec![22, 28, 8, 6])
        ); // [22,28,8,6,17,44]
    }
}
