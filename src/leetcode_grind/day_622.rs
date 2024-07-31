// https://leetcode.com/problems/filling-bookcase-shelves/description/?envType=daily-question&envId=2024-07-31
pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
    let n = books.len();
    let mut dp = vec![0; n + 1];
    dp[0] = 0;
    dp[1] = books[0][1];

    for i in 2..=n {
        let mut remaining_shelf_width = shelf_width - books[i - 1][0];
        let mut max_height = books[i - 1][1];
        dp[i] = books[i - 1][1] + dp[i - 1];

        let mut j = i - 1;

        while j > 0 && remaining_shelf_width - books[j - 1][0] >= 0 {
            max_height = max_height.max(books[j - 1][1]);
            remaining_shelf_width -= books[j - 1][0];
            dp[i] = dp[i].min(max_height + dp[j - 1]);
            j -= 1;
        }
    }
    dp[n]
}
