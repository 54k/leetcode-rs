// https://leetcode.com/problems/the-latest-time-to-catch-a-bus/description/
pub fn latest_time_catch_the_bus(
    mut buses: Vec<i32>,
    mut passengers: Vec<i32>,
    capacity: i32,
) -> i32 {
    use std::collections::HashSet;
    buses.sort();
    passengers.sort();
    let mut set = HashSet::new();
    let mut ans = 0;
    let mut j = 0;
    for i in 0..buses.len() {
        let mut c = 0;

        while j < passengers.len() && c < capacity && passengers[j] <= buses[i] {
            if !set.contains(&(passengers[j] - 1)) {
                ans = passengers[j] - 1;
            }

            set.insert(passengers[j]);
            j += 1;
            c += 1;
        }
        if c < capacity && !set.contains(&buses[i]) {
            ans = buses[i];
        }
    }
    ans
}
