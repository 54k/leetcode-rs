// todo https://leetcode.com/problems/strong-password-checker/description/
// todo https://leetcode.com/problems/replace-non-coprime-numbers-in-array/description/

// https://leetcode.com/problems/linked-list-random-node/description/
// https://leetcode.com/problems/linked-list-random-node/editorial/
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {
    head: Option<Box<ListNode>>,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(head: Option<Box<ListNode>>) -> Self {
        Self { head }
    }

    fn get_random(&self) -> i32 {
        use rand::{thread_rng, Rng};
        let mut scope = 1.0;
        let mut chosen_value = 0;
        let mut head = self.head.as_ref();
        while let Some(h) = head {
            let r = thread_rng().gen::<f64>();
            if r < 1.0 / scope {
                chosen_value = h.val;
            }
            scope += 1.0;
            head = h.next.as_ref();
        }
        chosen_value
    }
}

// https://leetcode.com/problems/find-all-k-distant-indices-in-an-array/description/
pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
    use std::collections::*;
    let mut set = HashSet::new();
    let mut key_idx = vec![];
    for i in 0..nums.len() {
        if nums[i] == key {
            key_idx.push(i as i32);
        }
    }
    for i in 0..nums.len() {
        for &j in &key_idx {
            if (i as i32 - j).abs() <= k {
                set.insert(i as i32);
            }
        }
    }

    let mut ans = set.into_iter().collect::<Vec<_>>();
    ans.sort();
    ans
}

// https://leetcode.com/problems/split-with-minimum-sum/description/
pub fn split_num(num: i32) -> i32 {
    // Assign digits to num1 and num2 alternatively.
    let mut num = num.to_string().chars().collect::<Vec<_>>();
    num.sort();
    let mut n1 = vec![];
    let mut n2 = vec![];
    for (i, &char) in num.iter().enumerate() {
        if i % 2 == 0 {
            n1.push(char);
        } else {
            n2.push(char);
        }
    }
    n1.into_iter().collect::<String>().parse::<i32>().unwrap()
        + n2.into_iter().collect::<String>().parse::<i32>().unwrap()
}

// https://leetcode.com/problems/task-scheduler/description/
// https://leetcode.com/problems/task-scheduler/solutions/104495/java-o-n-solution-beats-99-76-use-only-array-easy-understanding/
pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    // There is one crucial point for you:
    // the ONLY thing you need to care is the max number of one task!
    // We set apart each max task with interval n, and we hope to put all other tasks into those intervals.
    // If the number of those tasks exceeds the interval space, then we don't need any idle interval at all.
    // If not, the interval space plus the max tasks will be the least interval.
    // Be care for the existent of multiple max tasks.
    let mut storage = vec![0; 26];
    for i in 0..tasks.len() {
        storage[tasks[i] as usize - 'A' as usize] += 1;
    }
    let mut max = 0;
    let mut count = 1;
    for num in storage {
        if num == 0 {
            continue;
        }
        if max < num {
            max = num;
            count = 1;
        } else if max == num {
            count += 1;
        }
    }
    let space = (n + 1) * (max - 1) + count;
    if space < tasks.len() as i32 {
        tasks.len() as i32
    } else {
        space
    }
}

// https://leetcode.com/problems/task-scheduler-ii/
// https://leetcode.com/problems/task-scheduler-ii/solutions/2388089/c-concise-with-comments/
// https://leetcode.com/problems/task-scheduler-ii/solutions/2388101/hash-table-in-single-pass-with-explanation/?orderBy=most_relevant
pub fn task_scheduler_ii(tasks: Vec<i32>, space: i32) -> i64 {
    // Complete the task if the wait time is lower than the current day. Put the next day for it.
    // Otherwise, skip the days to the time where it can be completed
    // We could use a Hash table to see when we're allowed to do the task
    use std::collections::*;
    let mut last_day = HashMap::new();
    let mut ans = 0i64;
    for &t in &tasks {
        if last_day.contains_key(&t) {
            ans = ans.max(*last_day.get(&t).unwrap());
        }
        ans += 1;
        *last_day.entry(t).or_insert(0i64) = ans + space as i64;
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test338() {
        let mut sol = Solution::new(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        })));

        println!("{}", sol.get_random());
        println!("{}", sol.get_random());
        println!("{}", sol.get_random());
    }

    #[test]
    fn test339() {
        println!(
            "{:?}",
            find_k_distant_indices(vec![3, 4, 9, 1, 3, 9, 5], 9, 1)
        ); // [1,2,3,4,5,6]
        println!("{:?}", find_k_distant_indices(vec![2, 2, 2, 2, 2], 2, 2)); // [0,1,2,3,4]
    }

    #[test]
    fn test340() {
        println!("{:?}", split_num(4325)); // 59
        println!("{:?}", split_num(687)); // 75
    }

    #[test]
    fn test341() {
        println!(
            "{:?}",
            least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2)
        ); // 8
        println!(
            "{:?}",
            least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0)
        ); // 6
    }

    #[test]
    fn test342() {
        println!("{:?}", task_scheduler_ii(vec![1, 2, 1, 2, 3, 1], 3)); // 9
        println!("{:?}", task_scheduler_ii(vec![5, 8, 8, 5], 2)); // 6
    }
}
