pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
    let mut t = 0;
    let mut wt = 0i64;
    for c in &customers {
        t = c[0].max(t) + c[1];
        wt += t as i64 - c[0] as i64;
    }
    wt as f64 / customers.len() as f64
}
