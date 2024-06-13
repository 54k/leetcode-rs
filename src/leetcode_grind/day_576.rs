pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
    let mut seats = seats;
    let mut students = students;

    let mut cnt = vec![0; 101];
    for &s in &seats {
        cnt[s as usize] += 1;
    }
    let mut j = 0;
    for i in 0..=100 {
        while cnt[i] > 0 {
            cnt[i] -= 1;
            seats[j] = i as i32;
            j += 1;
        }
    }

    cnt = vec![0; 101];
    for &s in &students {
        cnt[s as usize] += 1;
    }

    j = 0;
    for i in 0..=100 {
        while cnt[i] > 0 {
            cnt[i] -= 1;
            students[j] = i as i32;
            j += 1;
        }
    }

    // println!("{:?}", seats);
    // println!("{:?}", students);
    let mut ans = 0;
    for i in 0..seats.len() {
        ans += (seats[i] - students[i]).abs();
    }
    ans
}
