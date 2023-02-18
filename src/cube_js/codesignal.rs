// CREATE PROCEDURE solution()
// BEGIN
// /* Write your SQL here. Terminate each statement with a semicolon. */
// select id as oldId, ROW_NUMBER () OVER (ORDER BY id) as newId from itemIds ids order by ids.id;
// END

// CREATE PROCEDURE solution()
// BEGIN
// /* Write your SQL here. Terminate each statement with a semicolon. */
// select fl.flight_id, (fl.number_of_seats - coalesce(p.occupied, 0)) as free_seats from
// (select fl.flight_id, pl.number_of_seats from flights fl left join planes pl on fl.plane_id = pl.plane_id) fl left join
// (select flight_id, count(seat_no) as occupied from purchases group by flight_id) p on fl.flight_id = p.flight_id
// order by fl.flight_id asc;
// END

fn solution_1(mut sequence: Vec<i32>) -> f64 {
    sequence.sort();
    if sequence.len() % 2 == 0 {
        (sequence[sequence.len() / 2 - 1] + sequence[sequence.len() / 2]) as f64 / 2f64
    } else {
        sequence[sequence.len() / 2] as f64
    }
}

fn solution_2(inputString: String) -> bool {
    let inputString = inputString.chars().collect::<Vec<_>>();
    let mut i = 0;
    let mut j = inputString.len() - 1;
    while i < j {
        if inputString[i] != inputString[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

fn solution_3(picture: Vec<String>) -> Vec<String> {
    let picture = picture
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut ans = vec![];
    let n = picture.len();
    let m = picture[0].len();

    for i in 0..n + 2 {
        let mut r = String::new();
        for j in 0..m + 2 {
            if i == 0 || j == 0 || i == n + 1 || j == m + 1 {
                r.push('*');
            } else {
                r.push(picture[i - 1][j - 1]);
            }
        }
        ans.push(r);
    }
    ans
}

fn solution_4(length: Vec<i32>, width: Vec<i32>, height: Vec<i32>) -> bool {
    use std::cmp::Ordering;
    let mut a = vec![];
    for i in 0..length.len() {
        let mut v = vec![length[i], width[i], height[i]];
        v.sort();
        a.push((v[0], v[1], v[2]));
    }
    a.sort_by(|x, y| {
        if x.0 < y.0 && x.1 < y.1 && x.2 < y.2 {
            return Ordering::Greater;
        }
        Ordering::Less
    });
    for i in 1..a.len() {
        let x = a[i - 1];
        let y = a[i];
        if x.0 <= y.0 || x.1 <= y.1 || x.2 <= y.2 {
            return false;
        }
    }
    true
}

fn solution_5(arr: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];
    for i in 2..arr.len() {
        let a = arr[i];
        let b = arr[i - 1];
        let c = arr[i - 2];
        ans.push((a > c - b && a > b - c && b > a - c) as i32);
    }
    ans
}

// http://www.cut-the-knot.org/pythagoras/DistanceFormula.shtml
fn solution_6(mut p: Vec<Vec<i32>>) -> f64 {
    fn eucl(p1: &[i32], p2: &[i32]) -> f64 {
        let x = p1[0] as i64;
        let y = p1[1] as i64;
        let a = p2[0] as i64;
        let b = p2[1] as i64;
        let f = x - a;
        let g = y - b;
        let k = (f * f) + (g * g);
        f64::sqrt(k as f64)
    }

    let mut ans = f64::MAX;

    for i in 0..p.len() {
        for j in i..p.len() {
            if i != j {
                let pi = &p[i];
                let pj = &p[j];
                let e = eucl(pi, pj).min(ans);
                ans = e;
            }
        }
    }
    ans
}

fn solution_7(a: String, b: String) -> String {
    let mut a: Vec<char> = a.chars().collect();
    let mut b: Vec<char> = b.chars().collect();

    if a.len() < b.len() {
        std::mem::swap(&mut a, &mut b);
    }

    let mut rem = 0;
    let mut res = vec![];

    while !a.is_empty() && !b.is_empty() {
        let ai = a.pop().unwrap() as i32 - 48;
        let bj = b.pop().unwrap() as i32 - 48;

        let sum = ai + bj + rem;
        res.push(if sum % 2 == 0 { 0 } else { 1 });
        rem = sum / 2;
    }

    while !a.is_empty() {
        let ai = a.pop().unwrap() as i32 - 48;
        let sum = ai + rem;
        res.push(if sum % 2 == 0 { 0 } else { 1 });
        rem = sum / 2;
    }

    if rem > 0 {
        res.push(rem);
    }

    res.into_iter()
        .rev()
        .map(|x| char::from_u32(x as u32 + 48).unwrap())
        .collect()
}

fn solution_8(fractions: Vec<String>) -> Vec<String> {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }
        gcd(b, ((a % b) + b) % b)
    }

    fn solve(a: &str, b: &str) -> String {
        let mut a = a
            .split('/')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut b = b
            .split('/')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let d = gcd(a[0], a[1]);
        a[0] /= d;
        a[1] /= d;

        let f = gcd(b[0], b[1]);
        b[0] /= f;
        b[1] /= f;

        let g = a[0] * b[1] + b[0] * a[1];
        let h = a[1] * b[1];
        let j = gcd(g, h);
        format!("{}/{}", g / j, h / j)
    }

    let mut ans = vec![];
    for f in fractions {
        let ss: Vec<&str> = f.split('+').collect();
        ans.push(solve(ss[0], ss[1]));
    }
    ans
}

fn solution_9(mut n: i32) -> i32 {
    let mut ans = 0;
    while n > 0 {
        ans += n % 10;
        n /= 10;
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        println!("{}", solution_1(vec![-1, 3, -2, 2]));
        println!("{}", solution_1(vec![1, 3, 2]));
    }

    #[test]
    fn test2() {
        println!("{}", solution_2("abac".to_string()));
        println!("{}", solution_2("a".to_string()));
    }

    #[test]
    fn test3() {
        println!(
            "{:?}",
            solution_3(vec!["abc".to_string(), "ded".to_string()])
        );
    }

    #[test]
    fn test4() {
        println!(
            "{:?}",
            solution_4(vec![1, 3, 2], vec![1, 3, 2], vec![1, 3, 2])
        ); // true

        println!(
            "{:?}",
            solution_4(vec![3, 1, 2], vec![3, 1, 2], vec![3, 2, 1])
        ); // false
    }

    #[test]
    fn test5() {
        println!("{:?}", solution_5(vec![1, 2, 2, 4]));
    }

    #[test]
    fn test6() {
        println!(
            "{:?}",
            solution_6(vec![vec![3, -3], vec![2, -1], vec![-2, 2], vec![-1, 0]])
        );
        println!("{:?}", solution_6(vec![vec![-2, 2], vec![-2, 2]]));
    }

    #[test]
    fn test7() {
        println!("{:?}", solution_7("1000".to_string(), "111".to_string()));
        println!("{:?}", solution_7("1".to_string(), "11".to_string()));
        println!("{:?}", solution_7("11".to_string(), "11".to_string()));
        println!("{:?}", solution_7("100".to_string(), "1".to_string()));
        println!(
            "{:?}",
            solution_7(
                solution_7("1111".to_string(), "1110".to_string()),
                "1011".to_string()
            )
        );
    }

    #[test]
    fn test8() {
        println!("{:?}", solution_8(vec!["2/3+5/7".to_string()])); // 14/21 + 15/21 29/21
        println!("{:?}", solution_8(vec!["2/4+1/2".to_string()])); // 1/2 + 1/2 1/1
    }

    #[test]
    fn test9() {
        println!("{}", solution_9(123));
        println!("{}", solution_9(13));
        println!("{}", solution_9(99));
    }
}
