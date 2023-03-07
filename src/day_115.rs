// https://leetcode.com/problems/minimum-time-to-complete-trips/editorial/
pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
    fn can_make_total_trips(time: &Vec<i32>, time_given: i64, total_trips: i32) -> bool {
        let mut trips = 0;
        for &t in time {
            trips += time_given / (t as i64);
        }
        trips >= total_trips as i64
    }

    let mut lo = 1;
    let mut hi = time.iter().copied().max().unwrap() as i64 * total_trips as i64;
    while lo < hi {
        let time_given = (lo + hi) / 2;
        if can_make_total_trips(&time, time_given, total_trips) {
            hi = time_given;
        } else {
            lo = time_given + 1;
        }
    }
    lo
}

//https://leetcode.com/problems/count-ways-to-group-overlapping-ranges/description/
pub fn count_ways(ranges: Vec<Vec<i32>>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/subarray-sum-equals-k/description/
// https://leetcode.com/problems/subarray-sum-equals-k/editorial/
pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    todo!()
}

// todo https://leetcode.com/problems/replace-non-coprime-numbers-in-array/description/

mod test {
    use super::*;

    #[test]
    fn test327() {
        println!("{}", minimum_time(vec![5, 10, 10], 9)); // 25
        println!("{}", minimum_time(vec![1, 2, 3], 5)); // 3
        println!("{}", minimum_time(vec![2], 1)); // 2
    }

    #[test]
    fn test328() {
        println!("{}", count_ways(vec![vec![6, 10], vec![5, 15]])); // 2
        println!(
            "{}",
            count_ways(vec![vec![1, 3], vec![10, 20], vec![2, 5], vec![4, 8]])
        ); // 4
    }
}
