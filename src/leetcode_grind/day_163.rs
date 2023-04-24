use std::collections::BinaryHeap;

// https://leetcode.com/problems/car-pooling/description/
pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
    let max_right = trips.iter().map(|x| x[2]).max().unwrap();
    let mut arr = vec![0; max_right as usize + 1];
    for trip in trips {
        let (value, left, right) = (trip[0], trip[1] as usize, trip[2] as usize);
        arr[left] += value;
        arr[right] -= value;
    }
    let mut cur = 0;
    for v in arr {
        cur += v;
        if cur > capacity {
            return false;
        }
    }
    true
}

// https://leetcode.com/problems/brightest-position-on-street/description/
pub fn brightest_position(lights: Vec<Vec<i32>>) -> i32 {
    let mut arr = vec![];
    for light in lights {
        let (pos, rad) = (light[0], light[1]);
        arr.push((pos - rad, 1));
        arr.push((pos + rad + 1, -1));
    }
    arr.sort();
    let mut ans = 0;
    let mut max = 0;
    let mut cur = 0;
    for (pos, val) in arr {
        cur += val;
        if cur > max {
            max = cur;
            ans = pos;
        }
    }
    ans
}

// https://leetcode.com/problems/meeting-rooms-ii/
pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
    fn heap_sort_approach(mut intervals: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        intervals.sort();
        heap.push(-intervals[0][1]);
        for i in 1..intervals.len() {
            let (s, e) = (intervals[i][0], intervals[i][1]);
            if -(*heap.peek().unwrap()) <= s {
                heap.pop();
            }
            heap.push(-e);
        }
        heap.len() as i32
    }
    fn chrono_ordering_approach(intervals: Vec<Vec<i32>>) -> i32 {
        let mut start = vec![];
        let mut end = vec![];
        for i in intervals {
            start.push(i[0]);
            end.push(i[1]);
        }
        start.sort();
        end.sort();
        let mut s = 0;
        let mut e = 0;
        let mut used_rooms = 0;
        while s < start.len() {
            if start[s] >= end[e] {
                e += 1;
                used_rooms -= 1;
            }
            s += 1;
            used_rooms += 1;
        }
        used_rooms
    }
    chrono_ordering_approach(intervals)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test462() {
        println!("{}", car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 4)); // false
        println!("{}", car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 5)); // true
    }

    #[test]
    fn test463() {
        println!(
            "{}",
            brightest_position(vec![vec![-3, 2], vec![1, 2], vec![3, 3]])
        ); // -1

        println!("{}", brightest_position(vec![vec![1, 0], vec![0, 1]])); // 1
    }

    #[test]
    fn test464() {
        println!(
            "{}",
            min_meeting_rooms(vec![vec![0, 30], vec![5, 10], vec![15, 20]])
        ); // 2
        println!("{}", min_meeting_rooms(vec![vec![7, 10], vec![2, 4]])); // 1
    }
}
