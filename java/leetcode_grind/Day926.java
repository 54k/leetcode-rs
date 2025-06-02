package leetcode_grind;

import java.util.Arrays;

public class Day926 {
    // https://leetcode.com/problems/candy/submissions/1651527856/?envType=daily-question&envId=2025-06-02
    static class Solution1 {
        public int candy(int[] ratings) {
            int[] candies = new int[ratings.length];
            Arrays.fill(candies, 1);
            boolean hasChanged = true;
            while (hasChanged) {
                hasChanged = false;
                for (int i = 0; i < ratings.length; i++) {
                    if (i != ratings.length - 1 && ratings[i] > ratings[i + 1] && candies[i] <= candies[i + 1]) {
                        candies[i] = candies[i + 1] + 1;
                        hasChanged = true;
                    }
                    if (i > 0 && ratings[i] > ratings[i - 1] && candies[i] <= candies[i - 1]) {
                        candies[i] = candies[i - 1] + 1;
                        hasChanged = true;
                    }
                }
            }
            int sum = 0;
            for (int candy : candies) {
                sum += candy;
            }
            return sum;
        }
    }

    static class Solution2 {
        public int candy(int[] ratings) {
            int sum = 0;
            int[] left2right = new int[ratings.length];
            int[] right2left = new int[ratings.length];
            Arrays.fill(left2right, 1);
            Arrays.fill(right2left, 1);

            for (int i = 1; i < ratings.length; i++) {
                if (ratings[i] > ratings[i - 1]) {
                    left2right[i] = left2right[i - 1] + 1;
                }
            }

            for (int i = ratings.length - 2; i >= 0; i--) {
                if (ratings[i] > ratings[i + 1]) {
                    right2left[i] = right2left[i + 1] + 1;
                }
            }
            for (int i = 0; i < ratings.length; i++) {
                sum += Math.max(left2right[i], right2left[i]);
            }
            return sum;
        }
    }

    static class Solution3 {
        public int candy(int[] ratings) {
            int[] candies = new int[ratings.length];
            Arrays.fill(candies, 1);
            for (int i = 1; i < ratings.length; i++) {
                if (ratings[i] > ratings[i - 1]) {
                    candies[i] = candies[i - 1] + 1;
                }
            }
            int sum = candies[ratings.length - 1];
            for (int i = ratings.length - 2; i >= 0; i--) {
                if (ratings[i] > ratings[i + 1]) {
                    candies[i] = Math.max(candies[i], candies[i + 1] + 1);
                }
                sum += candies[i];
            }
            return sum;
        }
    }

    static class Solution4 {
        int count(int n) {
            return (n * (n + 1)) / 2;
        }

        public int candy(int[] ratings) {
            if (ratings.length <= 1) {
                return ratings.length;
            }

            int candies = 0;
            int up = 0;
            int down = 0;
            int oldSlope = 0;

            for (int i = 1; i < ratings.length; i++) {
                int newSlope = (ratings[i] > ratings[i - 1]) ? 1 : (ratings[i] < ratings[i - 1] ? -1 : 0);

                if ((oldSlope > 0 && newSlope == 0) || (oldSlope < 0 && newSlope >= 0)) {
                    candies += count(up) + count(down) + Math.max(up, down);
                    up = 0;
                    down = 0;
                }

                if (newSlope > 0) {
                    up++;
                } else if (newSlope < 0) {
                    down++;
                } else {
                    candies++;
                }

                oldSlope = newSlope;
            }

            candies += count(up) + count(down) + Math.max(up, down) + 1;
            return candies;
        }
    }
}
