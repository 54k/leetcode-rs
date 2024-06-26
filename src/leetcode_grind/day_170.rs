// https://leetcode.com/problems/average-salary-excluding-the-minimum-and-maximum-salary/description/
pub fn average(mut salary: Vec<i32>) -> f64 {
    salary.sort();
    let acc = salary[1..salary.len() - 1]
        .into_iter()
        .enumerate()
        .fold((0, 0), |mut acc, v| {
            acc.0 = acc.0.max(v.0 + 1);
            acc.1 += v.1;
            acc
        });
    acc.1 as f64 / acc.0 as f64
}

// https://leetcode.com/problems/expressive-words/description/
pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
    fn can_express(s: &Vec<char>, w: &Vec<char>) -> bool {
        let mut i = 0;
        let mut j = 0;
        while i < s.len() && j < w.len() {
            let mut cnt_s = 1;
            let mut cnt_w = 1;
            if s[i] != w[j] {
                return false;
            }
            while i < s.len() - 1 && s[i] == s[i + 1] {
                cnt_s += 1;
                i += 1;
            }
            while j < w.len() - 1 && w[j] == w[j + 1] {
                cnt_w += 1;
                j += 1;
            }
            if cnt_s < cnt_w || ((cnt_s - cnt_w) > 0 && cnt_s < 3) {
                return false;
            }
            i += 1;
            j += 1;
        }
        i == s.len() && j == w.len()
    }

    let mut cnt = 0;
    for w in words {
        if can_express(&s[..].chars().collect(), &w[..].chars().collect()) {
            cnt += 1;
        }
    }
    cnt
}

pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut triangle = vec![];
    triangle.push(vec![1]);

    for row_num in 1..num_rows as usize {
        let mut row = vec![1];
        let prev_row = triangle[row_num - 1].clone();

        for j in 1..row_num {
            row.push(prev_row[j - 1] + prev_row[j]);
        }

        row.push(1);
        triangle.push(row);
    }

    triangle
}

// https://leetcode.com/problems/pascals-triangle-ii/description/
pub fn get_row(row_index: i32) -> Vec<i32> {
    fn generate(rows: usize) -> Vec<i32> {
        let mut last_row = vec![1];
        for rn in 1..=rows {
            let mut next_row = vec![1; rn + 1];
            for j in 1..rn {
                next_row[j] = last_row[j - 1] + last_row[j];
            }
            last_row = next_row;
        }
        last_row
    }
    generate(row_index as usize)
}

pub fn rotate(nums: &mut Vec<i32>, mut k: i32) {
    nums.reverse();
    k %= nums.len() as i32;
    let k = k as usize;
    nums[0..k].reverse();
    nums[k..].reverse();
}

// https://leetcode.com/problems/reverse-words-in-a-string-ii/description/
pub fn reverse_words_ii(s: &mut Vec<char>) {
    fn reverse_words(s: &mut Vec<char>) {
        let len = s.len();
        let mut j = 0;
        for i in 0..len {
            if s[i] == ' ' || i == len - 1 {
                s[j..if i == len - 1 { i + 1 } else { i }].reverse();
                j = i + 1;
            }
        }
    }
    s.reverse();
    reverse_words(s);
}

// https://leetcode.com/problems/reverse-words-in-a-string/description/
pub fn reverse_words(s: String) -> String {
    let s = s
        .split_ascii_whitespace()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                None
            } else {
                Some(format!("{} ", x))
            }
        })
        .rev()
        .flat_map(|x| x.chars().collect::<Vec<_>>())
        .collect::<String>();
    s.trim().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test471() {
        println!(
            "{:?}",
            expressive_words(
                "heeellooo".to_string(),
                vec!["hello".to_string(), "hi".to_string(), "helo".to_string()]
            )
        ); // 1
        println!(
            "{:?}",
            expressive_words(
                "zzzzzyyyyy".to_string(),
                vec!["zzyy".to_string(), "zy".to_string(), "zyy".to_string()]
            )
        ); // 3
        println!(
            "{:?}",
            expressive_words("abc".to_string(), vec!["abcd".to_string()])
        ); // 0
        println!(
            "{:?}",
            expressive_words("aaa".to_string(), vec!["aaaa".to_string()])
        ); // 0
    }

    #[test]
    fn test472() {
        println!("{:?}", generate(5)); // [[1], [1, 1], [1, 2, 1], [1, 3, 3, 1], [1, 4, 6, 4, 1]]
    }

    #[test]
    fn test473() {
        let mut s = "  hello world  ".chars().collect();
        reverse_words_ii(&mut s);
        println!("{:?}", s);
    }
}
