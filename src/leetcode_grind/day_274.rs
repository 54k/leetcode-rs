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
