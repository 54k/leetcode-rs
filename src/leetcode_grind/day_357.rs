// https://leetcode.com/problems/last-moment-before-all-ants-fall-out-of-a-plank/description/
pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut ans = 0;
    for num in left {
        ans = ans.max(num);
    }
    for num in right {
        ans = ans.max(n - num);
    }
    ans
}

// https://leetcode.com/problems/count-number-of-rectangles-containing-each-point/description/
pub fn count_rectangles(rectangles: Vec<Vec<i32>>, points: Vec<Vec<i32>>) -> Vec<i32> {
    fn bin_search(arr: &Vec<i32>, target: i32) -> i32 {
        let mut lo = 0;
        let mut hi = arr.len() as i32;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if arr[mid as usize] < target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        arr.len() as i32 - lo
    }

    use std::collections::HashMap;
    let mut map = HashMap::new();
    for rect in rectangles {
        map.entry(rect[1]).or_insert(vec![]).push(rect[0]);
    }
    for v in map.values_mut() {
        v.sort();
    }

    let mut counts = vec![];
    for point in points {
        let (x, y) = (point[0], point[1]);
        let mut count = 0;
        for i in y..=100 {
            if (!map.contains_key(&i)) {
                continue;
            }
            count += bin_search(map.get(&i).unwrap(), x);
        }
        counts.push(count);
    }
    counts
}

// https://leetcode.com/problems/frequency-tracker/description/
mod freq_tracker {
    use std::collections::HashMap;

    struct FrequencyTracker {
        freq: HashMap<i32, i32>,
        freq_lookup: HashMap<i32, i32>,
    }

    impl FrequencyTracker {
        fn new() -> Self {
            Self {
                freq: HashMap::new(),
                freq_lookup: HashMap::new(),
            }
        }

        fn add(&mut self, number: i32) {
            *self.freq.entry(number).or_insert(0) += 1;
            let f = self.freq[&number];

            *self.freq_lookup.entry(f).or_insert(0) += 1;
            *self.freq_lookup.entry(f - 1).or_insert(1) -= 1;

            if self.freq_lookup[&(f - 1)] == 0 {
                self.freq_lookup.remove(&(f - 1));
            }
        }

        fn delete_one(&mut self, number: i32) {
            if !self.freq.contains_key(&number) {
                return;
            }

            *self.freq.entry(number).or_insert(1) -= 1;
            let f = self.freq[&number];

            *self.freq_lookup.entry(f + 1).or_insert(1) -= 1;

            if self.freq_lookup[&(f + 1)] == 0 {
                self.freq_lookup.remove(&(f + 1));
            }

            if f == 0 {
                self.freq.remove(&number);
            } else {
                *self.freq_lookup.entry(f).or_insert(0) += 1;
            }
        }

        fn has_frequency(&self, frequency: i32) -> bool {
            self.freq_lookup.contains_key(&frequency)
        }
    }
}
