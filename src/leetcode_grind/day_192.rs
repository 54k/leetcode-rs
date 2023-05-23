use std::cmp::Reverse;
use std::collections::BinaryHeap;

// https://leetcode.com/problems/sequentially-ordinal-rank-tracker/description/
// https://leetcode.com/problems/sequentially-ordinal-rank-tracker/solutions/1632156/two-heaps/
struct SORTracker {
    max: BinaryHeap<(i32, Reverse<String>)>,
    min: BinaryHeap<Reverse<(i32, Reverse<String>)>>,
}

impl SORTracker {
    fn new() -> Self {
        Self {
            max: BinaryHeap::new(),
            min: BinaryHeap::new(),
        }
    }

    fn add(&mut self, name: String, score: i32) {
        self.min.push(Reverse((score, Reverse(name))));

        let (v, s) = self.min.pop().unwrap().0;
        self.max.push((v, s));
    }

    fn get(&mut self) -> String {
        let (v, s) = self.max.pop().unwrap();
        self.min.push(Reverse((v, s.clone())));
        s.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test507() {
        let mut tracker = SORTracker::new();
        tracker.add("bradford".to_string(), 2);
        tracker.add("branford".to_string(), 3);
        println!("{:?}", tracker.get());

        tracker.add("alps".to_string(), 2);
        println!("{:?}", tracker.get());

        tracker.add("orland".to_string(), 2);
        println!("{:?}", tracker.get());

        tracker.add("orlando".to_string(), 3);
        println!("{:?}", tracker.get());

        tracker.add("alpine".to_string(), 2);
        println!("{:?}", tracker.get());
        println!("{:?}", tracker.get());
    }
}
