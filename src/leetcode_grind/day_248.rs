// https://leetcode.com/problems/candy/description/
pub fn candy(ratings: Vec<i32>) -> i32 {
    pub fn candy_bruteforce_tle(ratings: Vec<i32>) -> i32 {
        let mut candies = vec![1; ratings.len()];
        let mut has_changed = true;
        while has_changed {
            has_changed = false;

            for i in 0..ratings.len() {
                if i != ratings.len() - 1
                    && ratings[i] > ratings[i + 1]
                    && candies[i] <= candies[i + 1]
                {
                    candies[i] = candies[i + 1] + 1;
                    has_changed = true;
                }

                if i > 0 && ratings[i] > ratings[i - 1] && candies[i] <= candies[i - 1] {
                    candies[i] = candies[i - 1] + 1;
                    has_changed = true;
                }
            }
        }

        candies.into_iter().sum::<i32>()
    }

    pub fn candy_2_arrays(ratings: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut left_2_right = vec![1; ratings.len()];
        let mut right_2_left = vec![1; ratings.len()];

        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                left_2_right[i] = left_2_right[i - 1] + 1;
            }
        }

        for i in (0..ratings.len() - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                right_2_left[i] = right_2_left[i + 1] + 1;
            }
        }

        for i in 0..ratings.len() {
            sum += left_2_right[i].max(right_2_left[i]);
        }

        sum
    }

    pub fn candy_1_array(ratings: Vec<i32>) -> i32 {
        let mut candies = vec![1; ratings.len()];

        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                candies[i] = candies[i - 1] + 1;
            }
        }

        let mut sum = candies[ratings.len() - 1];

        for i in (0..ratings.len() - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                candies[i] = candies[i].max(candies[i + 1] + 1);
            }
            sum += candies[i];
        }

        sum
    }

    pub fn candy_slope(ratings: Vec<i32>) -> i32 {
        fn count(n: i32) -> i32 {
            (n * (n + 1)) / 2
        }

        if ratings.len() <= 1 {
            return ratings.len() as i32;
        }

        let mut candies = 0;
        let mut up = 0;
        let mut down = 0;
        let mut old_slope = 0;
        for i in 1..ratings.len() {
            let new_slope = if ratings[i] > ratings[i - 1] {
                1
            } else if ratings[i] < ratings[i - 1] {
                -1
            } else {
                0
            };

            if (old_slope > 0 && new_slope == 0) || (old_slope < 0 && new_slope >= 0) {
                candies += count(up) + count(down) + up.max(down);
                up = 0;
                down = 0;
            }

            if new_slope > 0 {
                up += 1;
            } else if new_slope < 0 {
                down += 1;
            } else {
                candies += 1;
            }

            old_slope = new_slope;
        }

        candies += count(up) + count(down) + up.max(down) + 1;
        candies
    }

    candy_slope(ratings)
}

// https://leetcode.com/problems/one-edit-distance/description/
pub fn is_one_edit_distance(s: String, t: String) -> bool {
    fn is_one_edit_distance(mut s: String, mut t: String) -> bool {
        let ns = s.len();
        let nt = t.len();

        if ns > nt {
            return is_one_edit_distance(t, s);
        }

        if nt - ns > 1 {
            return false;
        }

        let (s, t) = (s.chars().collect::<Vec<_>>(), t.chars().collect::<Vec<_>>());
        for i in 0..ns {
            if s[i] != t[i] {
                if ns == nt {
                    return s[i + 1..] == t[i + 1..];
                } else {
                    return s[i..] == t[i + 1..];
                }
            }
        }

        ns + 1 == nt
    }
    is_one_edit_distance(s, t)
}
