// https://leetcode.com/problems/design-browser-history/
// https://leetcode.com/problems/design-browser-history/editorial/
struct BrowserHistory {
    history: Vec<String>,
    current_url: usize,
    last_url: usize,
}
impl BrowserHistory {
    fn new(homepage: String) -> Self {
        Self {
            history: vec![homepage],
            current_url: 0,
            last_url: 0,
        }
    }

    fn visit(&mut self, url: String) {
        self.current_url += 1;
        if self.current_url < self.history.len() {
            self.history[self.current_url] = url;
        } else {
            self.history.push(url);
        }
        self.last_url = self.current_url;
    }

    fn back(&mut self, steps: i32) -> String {
        self.current_url = 0.max(self.current_url as i32 - steps) as usize;
        self.history[self.current_url].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        self.current_url = self.last_url.min(self.current_url + steps as usize);
        self.history[self.current_url].clone()
    }
}

mod two_stacks {
    struct BrowserHistory {
        history: Vec<String>,
        future: Vec<String>,
        current: String,
    }
    impl BrowserHistory {
        fn new(homepage: String) -> Self {
            Self {
                history: vec![],
                future: vec![],
                current: homepage,
            }
        }

        fn visit(&mut self, url: String) {
            self.history.push(self.current.clone());
            self.future = vec![];
            self.current = url;
        }

        fn back(&mut self, mut steps: i32) -> String {
            while steps > 0 && !self.history.is_empty() {
                steps -= 1;
                self.future.push(self.current.clone());
                self.current = self.history.pop().unwrap();
            }
            self.current.clone()
        }

        fn forward(&mut self, mut steps: i32) -> String {
            while steps > 0 && !self.future.is_empty() {
                steps -= 1;
                self.history.push(self.current.clone());
                self.current = self.future.pop().unwrap();
            }
            self.current.clone()
        }
    }
}

// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/description/
pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
    fn negation_approach(mut nums: Vec<i32>) -> Vec<i32> {
        let mut dups = vec![];
        for i in 0..nums.len() {
            let next = nums[i].unsigned_abs();
            if nums[next as usize - 1] > 0 {
                nums[next as usize - 1] *= -1;
            }
        }
        for i in 0..nums.len() {
            if nums[i] > 0 {
                dups.push(i as i32 + 1);
            }
        }
        dups
    }
    fn cyclic_sort_approach(mut nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        let mut i = 0;
        while i < nums.len() {
            let correct = nums[i] - 1;
            if nums[i] != nums[correct as usize] {
                nums.swap(correct as usize, i);
            } else {
                i += 1;
            }
        }

        for i in 0..nums.len() {
            if nums[i] as usize != i + 1 {
                ans.push(i as i32 + 1);
            }
        }
        ans
    }
    negation_approach(nums)
}

// https://leetcode.com/problems/find-all-duplicates-in-an-array/
pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    fn cyclic_sort_approach(mut nums: Vec<i32>) -> Vec<i32> {
        let mut duplicates = vec![];
        let mut i = 0;
        while i < nums.len() {
            let correct_idx = nums[i] as usize - 1;
            if nums[correct_idx] != nums[i] {
                nums.swap(correct_idx, i);
            } else {
                i += 1;
            }
        }

        for i in 0..nums.len() {
            if nums[i] as usize - 1 != i {
                duplicates.push(nums[i]);
            }
        }
        duplicates
    }
    cyclic_sort_approach(nums)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test354() {
        println!(
            "{:?}",
            find_duplicates(vec![10, 2, 5, 10, 9, 1, 1, 4, 3, 7])
        ); // [10,1]

        println!("{:?}", find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1])); // [2,3]
        println!("{:?}", find_duplicates(vec![1, 1, 2])); // [1]
        println!("{:?}", find_duplicates(vec![1])); // []
    }

    #[test]
    fn test355() {
        println!(
            "{:?}",
            find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1])
        ); // [5,6]
        println!("{:?}", find_disappeared_numbers(vec![1, 1])); // [2]
    }
}
