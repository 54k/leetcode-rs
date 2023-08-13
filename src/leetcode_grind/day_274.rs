// https://leetcode.com/problems/data-stream-as-disjoint-intervals/description/
mod sum_ranges_sorted_set {
    use std::collections::BTreeMap;
    struct SummaryRanges {
        intervals: BTreeMap<i32, i32>,
    }

    impl SummaryRanges {
        fn new() -> Self {
            Self {
                intervals: BTreeMap::new(),
            }
        }

        fn add_num(&mut self, value: i32) {
            let mut left = value;
            let mut right = value;

            let floor = self.intervals.range(..=value).last();
            if let Some((l, r)) = floor {
                if *r >= value {
                    return;
                }
                if *r + 1 == value {
                    left = *l;
                }
            }

            let ceil = self.intervals.range(value + 1..).take(1).last();
            if let Some((l, r)) = ceil {
                if *l == value + 1 {
                    right = *r;
                    self.intervals.remove(&(value + 1));
                }
            }
            self.intervals.insert(left, right);
        }

        fn get_intervals(&self) -> Vec<Vec<i32>> {
            let mut ans = Vec::new();
            for (l, r) in &self.intervals {
                ans.push(vec![*l, *r]);
            }
            ans
        }
    }
}

// https://leetcode.com/problems/next-greater-element-i/description/
pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut stack = vec![];
    let mut map = HashMap::new();
    for i in 0..nums2.len() {
        while !stack.is_empty() && nums2[i] > *stack.last().unwrap() {
            map.insert(stack.pop().unwrap(), nums2[i]);
        }
        stack.push(nums2[i]);
    }

    while !stack.is_empty() {
        map.insert(stack.pop().unwrap(), -1);
    }

    let mut res = vec![0; nums1.len()];
    for i in 0..nums1.len() {
        res[i] = map[&nums1[i]];
    }
    res
}

// https://leetcode.com/problems/next-greater-element-iv/description/
pub fn second_greater_element(nums: Vec<i32>) -> Vec<i32> {
    pub fn second_greater_element_heap(nums: Vec<i32>) -> Vec<i32> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let n = nums.len();
        let mut middle = vec![vec![]; n];
        let mut stack = vec![];

        for i in 0..n {
            while !stack.is_empty() && nums[*stack.last().unwrap()] < nums[i] {
                middle[i].push(stack.pop().unwrap());
            }
            stack.push(i);
        }

        let mut ans = vec![-1; n];
        let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();

        for i in 0..n {
            while !heap.is_empty() && nums[heap.peek().unwrap().0 .1] < nums[i] {
                ans[heap.pop().unwrap().0 .1] = nums[i];
            }
            for &j in &middle[i] {
                heap.push(Reverse((nums[j], j)));
            }
        }

        ans
    }

    pub fn second_greater_element_2_stacks(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::VecDeque;
        let n = nums.len();
        let mut ans = vec![-1; n];

        let mut s1 = vec![];
        let mut s2 = vec![];

        for i in 0..n {
            while !s2.is_empty() && nums[i] > nums[*s2.last().unwrap()] {
                ans[s2.pop().unwrap()] = nums[i];
            }
            let mut temp = VecDeque::new();
            while !s1.is_empty() && nums[i] > nums[*s1.last().unwrap()] {
                temp.push_front(s1.pop().unwrap());
            }
            s2.extend(temp);
            s1.push(i);
        }
        ans
    }

    second_greater_element_2_stacks(nums)
}

#[test]
fn test_second_greater_element() {
    let res = second_greater_element(vec![2, 4, 0, 9, 6]);
    println!("{:?}", res);
}

// https://leetcode.com/problems/check-if-there-is-a-valid-partition-for-the-array/
pub fn valid_partition(nums: Vec<i32>) -> bool {
    use std::collections::HashMap;
    let mut memo = HashMap::new();
    memo.insert(-1, true);

    fn prefix_is_valid(nums: &Vec<i32>, i: i32, memo: &mut HashMap<i32, bool>) -> bool {
        if memo.contains_key(&i) {
            return memo[&i];
        }
        let mut ans = false;
        if i > 0 && nums[i as usize] == nums[i as usize - 1] {
            ans |= prefix_is_valid(nums, i - 2, memo);
        }
        if i > 1
            && nums[i as usize] == nums[i as usize - 1]
            && nums[i as usize - 1] == nums[i as usize - 2]
        {
            ans |= prefix_is_valid(nums, i - 3, memo);
        }
        if i > 1
            && nums[i as usize] == nums[i as usize - 1] + 1
            && nums[i as usize - 1] == nums[i as usize - 2] + 1
        {
            ans |= prefix_is_valid(nums, i - 3, memo);
        }
        memo.insert(i, ans);
        ans
    }

    prefix_is_valid(&nums, nums.len() as i32 - 1, &mut memo)
}

// https://leetcode.com/problems/check-if-there-is-a-valid-partition-for-the-array/
pub fn valid_partition_rolling_index_bottom_up(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let mut dp = vec![false; 3];
    dp[0] = true;

    for i in 0..n {
        let dp_idx = i + 1;
        let mut ans = false;

        if i > 0 && nums[i] == nums[i - 1] {
            ans |= dp[(dp_idx - 2) % 3];
        }
        if i > 1 && nums[i] == nums[i - 1] && nums[i] == nums[i - 2] {
            ans |= dp[(dp_idx - 3) % 3];
        }
        if i > 1 && nums[i] == nums[i - 1] + 1 && nums[i - 1] == nums[i - 2] + 1 {
            ans |= dp[(dp_idx - 3) % 3];
        }
        dp[dp_idx % 3] = ans;
    }

    dp[n % 3]
}

// https://leetcode.com/problems/sentence-screen-fitting/description/
pub fn words_typing(sentence: Vec<String>, rows: i32, cols: i32) -> i32 {
    let mut cursor = 0;
    let mut sb = String::new();
    for s in sentence {
        sb.push_str(s.as_str());
        sb.push(' ');
    }
    let sb = sb.chars().collect::<Vec<_>>();
    let n = sb.len() as i32;
    for _ in 0..rows {
        cursor += cols;

        while cursor % n >= 0 && sb[(cursor % n) as usize] != ' ' {
            cursor -= 1;
        }
        cursor += 1;
    }
    cursor / n
}

#[test]
fn test_words_typing() {
    let res = words_typing(vec!["hello".to_string(), "world".to_string()], 2, 8);
    println!("{}", res);
}

// https://leetcode.com/problems/text-justification/description/
pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    fn get_words(mut i: usize, words: &Vec<String>, max_width: i32) -> Vec<String> {
        let mut current_line = vec![];
        let mut current_len = 0;

        while i < words.len() && current_len + words[i].len() as i32 <= max_width {
            current_line.push(words[i].clone());
            current_len += words[i].len() as i32 + 1;
            i += 1;
        }

        current_line
    }

    fn create_line(mut line: Vec<String>, i: usize, words: &Vec<String>, max_width: i32) -> String {
        let mut base_len = -1;
        for word in &line {
            base_len += word.len() as i32 + 1;
        }

        let extra_spaces = max_width - base_len;

        if line.len() == 1 || i == words.len() {
            return line.join(" ") + &" ".repeat(extra_spaces as usize);
        }

        let word_count = line.len() as i32 - 1;
        let spaces_per_word = extra_spaces / word_count;
        let needs_extra_space = extra_spaces % word_count;

        for j in 0..needs_extra_space {
            line[j as usize] = line[j as usize].clone() + &" ";
        }

        for j in 0..word_count {
            line[j as usize] = line[j as usize].clone() + &" ".repeat(spaces_per_word as usize);
        }
        line.join(" ")
    }

    let mut ans = vec![];
    let mut i = 0;
    while i < words.len() {
        let curr_line = get_words(i, &words, max_width);
        i += curr_line.len();
        ans.push(create_line(curr_line, i, &words, max_width));
    }
    ans
}
