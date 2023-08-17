// https://leetcode.com/problems/maximum-sum-obtained-of-any-permutation/
pub fn max_sum_range_query(mut nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    const MOD: i64 = 1000000007;
    let n = nums.len();
    let mut count = vec![0; n];

    for r in requests {
        count[r[0] as usize] += 1;
        if r[1] + 1 < n as i32 {
            count[r[1] as usize + 1] -= 1;
        }
    }

    for i in 1..n {
        count[i] += count[i - 1];
    }

    nums.sort();
    count.sort();

    for i in 0..n {
        res += nums[i] as i64 * count[i] as i64;
    }
    (res % MOD) as i32
}

// https://leetcode.com/problems/maximum-average-pass-ratio/
pub fn max_average_ratio(mut classes: Vec<Vec<i32>>, mut extra_students: i32) -> f64 {
    fn ratio(c: &Vec<i32>) -> String {
        let ratio_before = c[0] as f64 / c[1] as f64;
        let ratio_after = (c[0] as f64 + 1.) / (c[1] as f64 + 1.);
        format!("{}", ratio_after - ratio_before)
    }

    fn sort(classes: &mut Vec<Vec<i32>>) {
        classes.sort_by(|a, b| ratio(&b).cmp(&ratio(&a)));
    }

    while extra_students > 0 {
        sort(&mut classes);
        // println!("{:?}", classes);
        classes[0][0] += 1;
        classes[0][1] += 1;
        extra_students -= 1;
    }

    let n = classes.len();
    classes
        .into_iter()
        .map(|x| x[0] as f64 / x[1] as f64)
        .fold(0.0, |mut acc, val| {
            acc += val;
            acc
        })
        / n as f64
}

#[test]
fn test_classes() {
    let res = max_average_ratio(vec![vec![1, 2], vec![3, 5], vec![2, 2]], 2);
    println!("{}", res);
}

// https://leetcode.com/problems/corporate-flight-bookings/
pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
    let mut seats = vec![0; n as usize];

    for b in bookings {
        let f = b[0] as usize - 1;
        let l = b[1] as usize - 1;
        let s = b[2];

        seats[f] += s;
        if l < seats.len() - 1 {
            seats[l + 1] += -s;
        }
    }

    // println!("{:?}", seats);

    for i in 1..seats.len() {
        seats[i] += seats[i - 1];
    }
    seats
}
