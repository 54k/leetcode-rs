// https://leetcode.com/problems/kth-distinct-string-in-an-array/description/?envType=daily-question&envId=2024-08-05
pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
    use std::collections::HashSet;
    let mut k = k;
    let mut distinct = HashSet::new();
    let mut unique = HashSet::new();

    for s in &arr {
        if distinct.contains(s) {
            continue;
        }

        if unique.contains(s) {
            unique.remove(s);
            distinct.insert(s.clone());
        } else {
            unique.insert(s.clone());
        }
    }

    for s in &arr {
        if unique.contains(s) {
            k -= 1;
            if k == 0 {
                return s.clone();
            }
        }
    }

    "".to_string()
}

// https://leetcode.com/problems/minimum-processing-time/description/
pub fn min_processing_time(mut processor_time: Vec<i32>, mut tasks: Vec<i32>) -> i32 {
    tasks.sort();
    tasks.reverse();
    processor_time.sort();
    let mut k = 0;
    let mut i = 0;
    let mut ans = i32::MIN;
    while i < tasks.len() {
        ans = ans.max(tasks[i] + processor_time[k]);
        k += 1;
        i += 4;
    }
    ans
}
