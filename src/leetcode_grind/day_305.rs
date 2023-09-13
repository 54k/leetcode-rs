// https://leetcode.com/problems/candy/
pub fn candy_i(ratings: Vec<i32>) -> i32 {
    let mut candies = vec![1; ratings.len()];
    let mut has_changed = true;

    while has_changed {
        has_changed = false;
        for i in 0..ratings.len() {
            if i != ratings.len() - 1 && ratings[i] > ratings[i + 1] && candies[i] <= candies[i + 1]
            {
                candies[i] = candies[i + 1] + 1;
                has_changed = true;
            }

            if i != 0 && ratings[i] > ratings[i - 1] && candies[i] <= candies[i - 1] {
                candies[i] = candies[i - 1] + 1;
                has_changed = true;
            }
        }
    }

    candies.into_iter().sum()
}

pub fn candy_ii(ratings: Vec<i32>) -> i32 {
    let mut left_to_right = vec![1; ratings.len()];
    for i in 1..ratings.len() {
        if ratings[i] > ratings[i - 1] && left_to_right[i] <= left_to_right[i - 1] {
            left_to_right[i] = left_to_right[i - 1] + 1;
        }
    }

    let mut right_to_left = vec![1; ratings.len()];
    for i in (0..ratings.len() - 1).rev() {
        if ratings[i] > ratings[i + 1] && right_to_left[i] <= right_to_left[i + 1] {
            right_to_left[i] = right_to_left[i + 1] + 1;
        }
    }

    let mut sum = 0;
    for i in 0..ratings.len() {
        sum += left_to_right[i].max(right_to_left[i]);
    }

    sum
}

pub fn candy_iii(ratings: Vec<i32>) -> i32 {
    let mut candies = vec![1; ratings.len()];
    for i in 1..ratings.len() {
        if ratings[i] > ratings[i - 1] {
            candies[i] = candies[i - 1] + 1;
        }
    }
    let mut sum = candies[candies.len() - 1];
    for i in (0..ratings.len() - 1).rev() {
        if ratings[i] > ratings[i + 1] && candies[i] <= candies[i + 1] {
            candies[i] = candies[i + 1] + 1;
        }
        sum += candies[i];
    }
    sum
}
