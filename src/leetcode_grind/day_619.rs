// https://leetcode.com/problems/longest-substring-without-repeating-characters/description/
pub fn length_of_longest_substring(s: String) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let mut chars = vec![None; 128];
    let mut left = 0;
    let mut right = 0;
    let mut res = 0;
    while right < s.len() {
        let r = s[right];
        let idx = chars[r as usize];

        if let Some(i) = idx {
            if i >= left && i < right {
                left = i + 1;
            }
        }

        res = res.max(right - left + 1);
        chars[r as usize] = Some(right);
        right += 1;
    }
    res as i32
}

// https://leetcode.com/problems/container-with-most-water/description/
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut left = 0;
    let mut right = height.len() - 1;

    while left < right {
        let width = right - left;
        max_area = max_area.max(height[left].min(height[right]) * width as i32);

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }
    max_area
}

// https://leetcode.com/problems/3sum/description/
pub fn three_sum_i(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn two_sum(nums: &Vec<i32>, i: usize, res: &mut Vec<Vec<i32>>) {
        use std::collections::HashSet;
        let mut seen = HashSet::new();
        let mut j = i + 1;
        while j < nums.len() {
            let complement = -nums[i] - nums[j];
            if seen.contains(&complement) {
                res.push(vec![nums[i], nums[j], complement]);
                while j + 1 < nums.len() && nums[j] == nums[j + 1] {
                    j += 1;
                }
            }
            seen.insert(nums[j]);
            j += 1;
        }
    }

    let mut nums = nums;
    nums.sort();
    let mut res = vec![];

    for i in 0..nums.len() {
        if nums[i] > 0 {
            break;
        }

        if i == 0 || nums[i - 1] != nums[i] {
            two_sum(&nums, i, &mut res);
        }
    }

    res
}

pub fn three_sum_ii(nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::{HashMap, HashSet};
    let mut res = HashSet::new();
    let mut dups = HashSet::new();
    let mut seen = HashMap::new();

    for i in 0..nums.len() {
        if dups.insert(nums[i]) {
            for j in i + 1..nums.len() {
                let complement = -nums[i] - nums[j];
                if seen.contains_key(&complement) && *seen.get(&complement).unwrap() == i {
                    let mut triplet = vec![nums[i], nums[j], complement];
                    triplet.sort();
                    res.insert(triplet);
                }
                seen.insert(nums[j], i);
            }
        }
    }
    res.into_iter().collect()
}

// https://leetcode.com/problems/next-permutation/description/
pub fn next_permutation(nums: &mut Vec<i32>) {
    let n = nums.len();
    let mut nums = nums;
    let mut pivot = n - 1;

    while pivot >= 1 && nums[pivot] <= nums[pivot - 1] {
        pivot -= 1;
    }

    if pivot != 0 {
        let mut j = n - 1;
        while nums[j] <= nums[pivot - 1] {
            j -= 1;
        }
        nums.swap(pivot - 1, j);
    }

    nums[pivot..n].reverse();
}

// https://leetcode.com/problems/multiply-strings/description/
pub fn multiply(num1: String, num2: String) -> String {
    if num1 == "0" || num2 == "0" {
        return "0".to_string();
    }
    let mut first_number = num1.chars().collect::<Vec<_>>();
    let mut second_number = num2.chars().collect::<Vec<_>>();

    first_number.reverse();
    second_number.reverse();

    let n = first_number.len() + second_number.len();
    let mut answer = vec!['0'; n];

    for place2 in 0..second_number.len() {
        let digit2 = second_number[place2] as i32 - '0' as i32;
        for place1 in 0..first_number.len() {
            let digit1 = first_number[place1] as i32 - '0' as i32;
            let current_pos = place1 + place2;

            let mut carry = answer[current_pos] as i32 - '0' as i32;
            let multiplication = digit1 * digit2 + carry;

            answer[current_pos] = ((multiplication % 10) as u8 + b'0') as char;
            let value = answer[current_pos + 1] as u32 - '0' as u32
                + (multiplication / 10) as u32
                + '0' as u32;
            answer[current_pos + 1] = char::from_u32(value).unwrap();
        }
    }

    if answer[answer.len() - 1] == '0' {
        answer.pop();
    }
    answer.reverse();
    answer.into_iter().collect()
}

// https://leetcode.com/problems/rotate-image/description/
pub fn rotate_i(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    for i in 0..(n + 1) / 2 {
        for j in 0..n / 2 {
            let tmp = matrix[n - 1 - j][i];
            matrix[n - 1 - j][i] = matrix[n - 1 - i][n - 1 - j];
            matrix[n - 1 - i][n - 1 - j] = matrix[j][n - 1 - i];
            matrix[j][n - 1 - i] = matrix[i][j];
            matrix[i][j] = tmp;
        }
    }
}

pub fn rotate_ii(matrix: &mut Vec<Vec<i32>>) {
    fn transponse(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n {
            for j in i + 1..n {
                let tmp = matrix[j][i];
                matrix[j][i] = matrix[i][j];
                matrix[i][j] = tmp;
            }
        }
    }

    fn reflect(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n {
            for j in 0..n / 2 {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[i][n - j - 1];
                matrix[i][n - j - 1] = tmp;
            }
        }
    }

    transponse(matrix);
    reflect(matrix);
}

// https://leetcode.com/problems/jump-game/description/
pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut leftmost = nums.len() - 1;
    for i in (0..nums.len() - 1).rev() {
        if nums[i] as usize + i >= leftmost {
            leftmost = i;
        }
    }
    leftmost == 0
}

// https://leetcode.com/problems/plus-one/description/
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];
    let mut carry = 1;
    for i in (0..digits.len()).rev() {
        let d = digits[i];
        let sum = d + carry;
        ans.push(sum % 10);
        carry = sum / 10;
    }
    if carry > 0 {
        ans.push(carry);
    }
    ans.into_iter().rev().collect()
}

pub fn plus_one_ii(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    let n = digits.len();

    for idx in (0..n).rev() {
        if digits[idx] == 9 {
            digits[idx] = 0;
        } else {
            digits[idx] += 1;
            return digits;
        }
    }

    digits = vec![0; n + 1];
    digits[0] = 1;
    digits
}

// https://leetcode.com/problems/minimum-window-substring/description/
pub fn min_window(s: String, t: String) -> String {
    use std::collections::HashMap;

    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();

    if s.len() == 0 || t.len() == 0 {
        return "".to_string();
    }

    let mut dict_t = HashMap::new();

    for i in 0..t.len() {
        *dict_t.entry(t[i]).or_insert(0) += 1;
    }

    let required = dict_t.len();
    let mut filtered_s = vec![];
    for i in 0..s.len() {
        if dict_t.contains_key(&s[i]) {
            filtered_s.push((i, s[i]));
        }
    }

    let mut l = 0;
    let mut r = 0;
    let mut formed = 0;
    let mut window_counts = HashMap::new();

    let mut ans = (usize::MAX, 0, 0);

    while r < filtered_s.len() {
        let mut c = filtered_s[r].1;
        *window_counts.entry(c).or_insert(0) += 1;

        if dict_t.contains_key(&c) && window_counts.get(&c) == dict_t.get(&c) {
            formed += 1;
        }

        while l <= r && formed == required {
            c = filtered_s[l].1;

            let end = filtered_s[r].0;
            let start = filtered_s[l].0;

            if end - start + 1 < ans.0 {
                ans.0 = end - start + 1;
                ans.1 = start;
                ans.2 = end;
            }

            *window_counts.get_mut(&c).unwrap() -= 1;
            if dict_t.contains_key(&c) && window_counts[&c] < dict_t[&c] {
                formed -= 1;
            }
            l += 1;
        }

        r += 1;
    }
    if ans.0 == usize::MAX {
        "".to_string()
    } else {
        s[ans.1..=ans.2].iter().copied().collect()
    }
}
