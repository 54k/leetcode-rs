// https://leetcode.com/problems/search-a-2d-matrix/
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let (m, n) = (matrix.len() as i32, matrix[0].len() as i32);
    let (mut left, mut right) = (0i32, (m * n - 1) as i32);

    while left <= right {
        let mid = left + (right - left) / 2;
        let r = mid / n;
        let c = mid % n;
        if matrix[r as usize][c as usize] == target {
            return true;
        } else if matrix[r as usize][c as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    false
}

// https://leetcode.com/problems/find-all-good-indices/description/
pub fn good_indices(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    let mut p1 = vec![1; n];
    let mut p2 = vec![1; n];

    for i in 1..n {
        if nums[i - 1] >= nums[i] {
            p1[i] = p1[i - 1] + 1;
        }
    }

    for i in (0..n - 1).rev() {
        if nums[i + 1] >= nums[i] {
            p2[i] = p2[i + 1] + 1;
        }
    }

    let mut ans = vec![];
    for i in k as usize..n - k as usize {
        if p1[i - 1] >= k && p2[i + 1] >= k {
            ans.push(i as i32);
        }
    }
    ans
}

// https://leetcode.com/problems/valid-palindrome-ii/description/
pub fn valid_palindrome(s: String) -> bool {
    fn is_palindrome(s: &Vec<char>, mut i: usize, mut j: usize) -> bool {
        while i < j {
            if s[i] != s[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
    let (mut i, mut j) = (0, s.len() - 1);
    let s = s.chars().collect::<Vec<_>>();
    while i < j {
        if s[i] != s[j] {
            return is_palindrome(&s, i + 1, j) || is_palindrome(&s, i, j - 1);
        }
        i += 1;
        j -= 1;
    }

    true
}
