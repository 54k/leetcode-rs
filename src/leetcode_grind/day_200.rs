// https://leetcode.com/problems/design-underground-system/description/
use std::collections::HashMap;

struct UndergroundSystem {
    check_in: HashMap<i32, (String, i32)>,
    trips: HashMap<String, (f64, f64)>,
}

impl UndergroundSystem {
    fn new() -> Self {
        Self {
            check_in: HashMap::new(),
            trips: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.check_in.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let (start_station, check_in_time) = self.check_in.get(&id).unwrap();
        let route_key = UndergroundSystem::get_station_key(start_station.clone(), station_name);
        let mut trip = self.trips.entry(route_key).or_insert((0.0, 0.0));
        trip.0 += t as f64 - *check_in_time as f64;
        trip.1 += 1.0;
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let route_key = UndergroundSystem::get_station_key(start_station, end_station);
        let route = self.trips[&route_key];
        route.0 / route.1
    }

    fn get_station_key(start_station: String, end_station: String) -> String {
        format!("{}->{}", start_station, end_station)
    }
}

// https://leetcode.com/problems/power-of-two/description/
pub fn is_power_of_two(n: i32) -> bool {
    let n = n as i64;
    if n != 0 {
        n & (-n) == n
    } else {
        false
    }
}

// https://leetcode.com/problems/k-radius-subarray-averages/description/
pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
    if k == 0 {
        return nums;
    }
    let mut prefix = vec![0];
    for &n in &nums {
        let l = *prefix.last().unwrap();
        prefix.push(l + n as i64);
    }
    let k = k as usize;
    let n = nums.len();
    let mut ans = vec![-1; n];

    if 2 * k + 1 > n {
        return ans;
    }

    for i in k..nums.len() - k {
        ans[i] = ((prefix[i + 1 + k] - prefix[i - k]) / (2 * k as i64 + 1)) as i32;
    }
    ans
}