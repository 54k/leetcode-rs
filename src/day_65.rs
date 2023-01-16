pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    intervals.push(new_interval);
    intervals.sort();
    let mut start = intervals[0][0];
    let mut end = intervals[0][1];
    let mut res = vec![];
    for i in intervals.iter().skip(1) {
        if i[0] <= end {
            end = end.max(i[1]);
        } else {
            res.push(vec![start, end]);
            start = i[0];
            end = i[1];
        }
    }
    res.push(vec![start, end]);
    res
}
