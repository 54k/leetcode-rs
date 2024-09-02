// https://leetcode.com/problems/find-the-student-that-will-replace-the-chalk/description/?envType=daily-question&envId=2024-09-02
pub fn chalk_replacer(chalk: Vec<i32>, mut k: i32) -> i32 {
    let n = chalk.len();
    let mut sum = 0;
    for i in 0..n {
        sum += chalk[i] as i64;
        if sum > k as i64 {
            break;
        }
    }
    k = k % (sum as i32);
    for i in 0..n {
        if k < chalk[i] {
            return i as i32;
        }
        k -= chalk[i];
    }
    0
}
