// https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/description
pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    let word1 = word1
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let word2 = word2
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut w1 = 0;
    let mut w2 = 0;
    let mut s1 = 0;
    let mut s2 = 0;

    while s1 < word1.len() && s2 < word2.len() {
        if word1[s1][w1] != word2[s2][w2] {
            return false;
        }

        w1 += 1;
        w2 += 1;

        if w1 == word1[s1].len() {
            w1 = 0;
            s1 += 1;
        }

        if w2 == word2[s2].len() {
            w2 = 0;
            s2 += 1;
        }
    }

    s1 == word1.len() && s2 == word2.len()
}

// https://leetcode.com/problems/queries-on-number-of-points-inside-a-circle/description/
pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    fn lower_bound(points: &Vec<Vec<i32>>, target: i32) -> i32 {
        let mut lo = -1;
        let mut hi = points.len() as i32;
        while lo + 1 < hi {
            let mid = (lo + hi) / 2;
            if points[mid as usize][0] < target {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        hi
    }

    let mut points = points;
    points.sort();

    let mut ans = vec![0; queries.len()];

    for (i, circle) in queries.into_iter().enumerate() {
        let from = lower_bound(&points, circle[0] - circle[2]);

        // println!("{:?}, {}, {}", points, from, circle[0]);

        if from == -1 || from == points.len() as i32 {
            continue;
        }

        for j in from as usize..points.len() {
            let point = &points[j];
            let dist = (circle[0] - point[0]).pow(2) + (circle[1] - point[1]).pow(2);
            if (dist <= circle[2].pow(2)) {
                ans[i] += 1;
            }
        }
    }

    ans
}
