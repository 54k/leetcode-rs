package leetcode_grind;

public class Day644 {
    // https://leetcode.com/problems/number-complement/description/?envType=daily-question&envId=2024-08-22
    static class Solution1 {
        public int findComplement(int num) {
            int n = (int) (Math.log(num) / Math.log(2)) + 1;
            int bitmask = (1 << n) - 1;
            return num ^ bitmask;
        }
    }

    static class Solution2 {
        public int findComplement(int num) {
            return (Integer.highestOneBit(num) << 1) - num - 1;
        }
    }

    // https://leetcode.com/problems/remove-boxes/description/
    static class Solution3 {
        public int removeBoxes(int[] boxes) {
            int[][][] dp = new int[100][100][100];
            return calculatePoints(boxes, dp, 0, boxes.length - 1, 0);
        }

        int calculatePoints(int[] boxes, int[][][] dp, int l, int r, int k) {
            if (l > r)
                return 0;

            while (r > l && boxes[r] == boxes[r - 1]) {
                r--;
                k++;
            }

            if (dp[l][r][k] != 0) {
                return dp[l][r][k];
            }

            dp[l][r][k] = calculatePoints(boxes, dp, l, r - 1, 0) + (k + 1) * (k + 1);
            for (int i = l; i < r; i++) {
                if (boxes[i] == boxes[r]) {
                    dp[l][r][k] = Math.max(dp[l][r][k],
                            calculatePoints(boxes, dp, l, i, k + 1) + calculatePoints(boxes, dp, i + 1, r - 1, 0));
                }
            }

            return dp[l][r][k];
        }
    }
}
