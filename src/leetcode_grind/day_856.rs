// https://leetcode.com/problems/count-days-without-meetings/description/?envType=daily-question&envId=2025-03-24
pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
    let mut free_days = 0;
    let mut latest_end = 0;
    meetings.sort();

    for meeting in meetings {
        let start = meeting[0];
        let end = meeting[1];

        if start > latest_end + 1 {
            free_days += start - latest_end - 1;
        }

        latest_end = latest_end.max(end);
    }

    free_days += days - latest_end;
    free_days
}