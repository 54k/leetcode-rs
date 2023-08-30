// https://leetcode.com/problems/maximal-range-that-each-element-is-maximum-in-it/
pub fn maximum_length_of_ranges(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![1; nums.len()];
    let n = nums.len();
    let mut stack = vec![];

    stack.push(-1);
    for i in 0..n {
        while stack[stack.len() - 1] != -1 && nums[stack[stack.len() - 1] as usize] < nums[i] {
            stack.pop();
        }
        ans[i] = i as i32 - stack[stack.len() - 1];
        stack.push(i as i32);
    }

    stack.clear();
    stack.push(n as i32);

    for i in (0..n).rev() {
        while (stack[stack.len() - 1] != n as i32)
            && nums[stack[stack.len() - 1] as usize] < nums[i]
        {
            stack.pop();
        }
        ans[i] += stack[stack.len() - 1] - i as i32 - 1;
        stack.push(i as i32);
    }

    ans
}

// https://leetcode.com/problems/minimum-replacements-to-sort-the-array/description/
pub fn minimum_replacement(nums: Vec<i32>) -> i64 {
    let mut ans = 0;
    let mut nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
    for i in (0..nums.len() - 1).rev() {
        if nums[i] <= nums[i + 1] {
            continue;
        }

        let num_elements = (nums[i] + nums[i + 1] - 1) / nums[i + 1];

        ans += num_elements - 1;

        nums[i] = nums[i] / num_elements;
    }
    ans
}

// https://leetcode.com/problems/find-permutation/description/
pub fn find_permutation(s: String) -> Vec<i32> {
    fn rev(res: &mut [i32], start: usize, end: usize) {
        for i in 0..(end - start) / 2 {
            res.swap(i + start, end - i - 1);
        }
    }

    let s = s.chars().collect::<Vec<_>>();
    let mut res = vec![0; s.len() + 1];
    for i in 0..res.len() as i32 {
        res[i as usize] = i + 1;
    }

    let mut i = 1;
    while i <= s.len() {
        let j = i;
        while i <= s.len() && s[i - 1] == 'D' {
            i += 1;
        }
        rev(&mut res, j - 1, i);
        i += 1;
    }
    res
}
