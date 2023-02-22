// https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/
// https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/solutions/2934252/capacity-to-ship-packages-within-d-days/
pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
    fn is_feasible(weights: &Vec<i32>, c: i32, days: i32) -> bool {
        let mut days_needed = 1;
        let mut current_load = 0;
        for &weight in weights {
            current_load += weight;
            if current_load > c {
                days_needed += 1;
                current_load = weight;
            }
        }
        days_needed <= days
    }

    let mut max_load = 0;
    let mut total_load = 0;
    for &weight in weights.iter() {
        total_load += weight;
        max_load = max_load.max(weight);
    }

    let mut lo = max_load;
    let mut hi = total_load;

    while lo < hi {
        let mid = (lo + hi) / 2;
        if is_feasible(&weights, mid, days) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

// https://leetcode.com/problems/count-integers-in-intervals/solutions/2039706/merge-intervals/
// https://leetcode.com/problems/count-integers-in-intervals/description/
struct CountIntervals {}

impl CountIntervals {
    fn new() -> Self {
        Self {}
    }

    fn add(&self, left: i32, right: i32) {}

    fn count(&self) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test268() {
        println!(
            "{}",
            ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5)
        ); // 15

        println!("{}", ship_within_days(vec![3, 2, 2, 4, 1, 4], 3)); // 6
        println!("{}", ship_within_days(vec![1, 2, 3, 1, 1], 4)); // 3
    }
}
