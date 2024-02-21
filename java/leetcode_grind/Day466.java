package leetcode_grind;

import java.util.Arrays;

public class Day466 {
    // https://leetcode.com/problems/greatest-sum-divisible-by-three/
    static class Solution1 {
        public int maxSumDivThree(int[] nums) {
            // dp[pos][mod]
            // Arrays.sort(nums);
            int[][] dp = new int[nums.length + 1][3];
            for (int i = nums.length - 1; i >= 0; i--) {
                dp[i] = dp[i + 1].clone();
                for (int j = 0; j < 3; j++) {
                    int idx = (dp[i + 1][j] + nums[i]) % 3;
                    dp[i][idx] = Math.max(dp[i][idx], dp[i + 1][j] + nums[i]);
                }
            }
            return dp[0][0];
        }
    }

    // https://leetcode.com/problems/stickers-to-spell-word/
    static class Solution2 {
        public int minStickers(String[] stickers, String target) {
            int N = target.length();
            int[] dp = new int[1 << N];
            Arrays.fill(dp, -1);
            dp[0] = 0;

            for (int state = 0; state < 1 << N; state++) {
                if (dp[state] == -1)
                    continue;
                for (String sticker : stickers) {
                    int now = state;

                    for (char letter : sticker.toCharArray()) {
                        for (int i = 0; i < N; i++) {
                            if (((now >> i) & 1) == 1)
                                continue;
                            if (target.charAt(i) == letter) {
                                now |= 1 << i;
                                break;
                            }
                        }
                    }
                    if (dp[now] == -1 || dp[now] > dp[state] + 1) {
                        dp[now] = dp[state] + 1;
                    }
                }
            }

            return dp[(1 << N) - 1];
        }
    }
}
