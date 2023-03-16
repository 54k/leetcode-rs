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

// https://leetcode.com/problems/koko-eating-bananas/
pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let max = piles.iter().max().unwrap();
    fn can_eat(piles: &[i32], eat_per_hour: i32, hours: i32) -> bool {
        let mut hours_taken = 0;
        for &pile in piles.iter() {
            if pile % eat_per_hour == 0 {
                hours_taken += pile / eat_per_hour;
            } else {
                hours_taken += pile / eat_per_hour + 1;
            }
        }
        hours_taken <= hours
    }
    let mut lo = 1;
    let mut hi = *max;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if can_eat(&piles, mid, h) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

// https://leetcode.com/problems/minimum-number-of-days-to-make-m-bouquets/
pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
    fn can_get_bouquets(bloom_day: &Vec<i32>, m: i32, k: i32, mid: i32) -> bool {
        let mut bouquets = 0;
        let mut count = 0;
        for i in 0..bloom_day.len() {
            if bloom_day[i] <= mid {
                count += 1;
                if count == k {
                    bouquets += 1;
                    count = 0;
                }
            } else {
                count = 0;
            }
        }
        bouquets >= m
    }
    let mut lo = 0;
    let mut hi = bloom_day.iter().copied().max().unwrap();
    let mut ans = 0;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        if can_get_bouquets(&bloom_day, m, k, mid) {
            ans = mid;
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }
    if ans > 0 {
        ans
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test273() {
        println!(
            "{}",
            ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5)
        ); // 15

        println!("{}", ship_within_days(vec![3, 2, 2, 4, 1, 4], 3)); // 6
        println!("{}", ship_within_days(vec![1, 2, 3, 1, 1], 4)); // 3
    }

    #[test]
    fn test274() {
        println!("{}", min_eating_speed(vec![3, 6, 7, 11], 8)); // 4
        println!("{}", min_eating_speed(vec![30, 11, 23, 4, 20], 5)); // 30
        println!("{}", min_eating_speed(vec![30, 11, 23, 4, 20], 6)); // 23
        println!("{}", min_eating_speed(vec![312884470], 312884469)); // 2
        println!(
            "{}",
            min_eating_speed(
                vec![
                    332484035, 524908576, 855865114, 632922376, 222257295, 690155293, 112677673,
                    679580077, 337406589, 290818316, 877337160, 901728858, 679284947, 688210097,
                    692137887, 718203285, 629455728, 941802184,
                ],
                823855818,
            )
        ); // 112677673
    }

    #[test]
    fn test275() {
        println!("{}", min_days(vec![1, 10, 3, 10, 2], 3, 1)); // 3
        println!("{}", min_days(vec![1, 10, 3, 10, 2], 3, 2)); // -1
        println!("{}", min_days(vec![7, 7, 7, 7, 12, 7, 7], 2, 3)); // 12
    }
}
