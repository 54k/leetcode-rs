// https://leetcode.com/problems/data-stream-as-disjoint-intervals/solutions/2866931/data-stream-as-disjoint-intervals/?orderBy=most_relevant
use std::collections::BTreeSet;

struct SummaryRanges {
    points: BTreeSet<i32>,
}

impl SummaryRanges {
    fn new() -> Self {
        Self {
            points: BTreeSet::new(),
        }
    }

    fn add_num(&mut self, value: i32) {
        self.points.insert(value);
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        if self.points.is_empty() {
            return vec![vec![0; 2]];
        }
        let mut ans = vec![];
        let mut left = self
            .points
            .iter()
            .take(1)
            .collect::<Vec<_>>()
            .first()
            .map(|f| **f)
            .unwrap();
        let mut right = left;
        for &point in self.points.iter().skip(1) {
            if point - right == 1 {
                right = point;
            } else {
                ans.push(vec![left, right]);
                left = point;
                right = left;
            }
        }
        ans.push(vec![left, right]);
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test180() {
        let mut ranges = SummaryRanges::new();
        ranges.add_num(1);
        println!("{:?}", ranges.get_intervals());
        ranges.add_num(3);
        println!("{:?}", ranges.get_intervals());
        ranges.add_num(7);
        println!("{:?}", ranges.get_intervals());
        ranges.add_num(2);
        println!("{:?}", ranges.get_intervals());
        ranges.add_num(6);
        println!("{:?}", ranges.get_intervals()); // [1,3],[6,7]
    }
}
