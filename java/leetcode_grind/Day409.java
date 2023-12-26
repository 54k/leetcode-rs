package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

public class Day409 {
    // https://leetcode.com/problems/number-of-dice-rolls-with-target-sum/description/
    static class Solution1 {
        final int MOD = 1_000_000_007;

        int waysToTarget(Integer[][] memo, int diceIndex, int n, int currSum, int target, int k) {
            if (diceIndex == n) {
                return currSum == target ? 1 : 0;
            }

            if (memo[diceIndex][currSum] != null) {
                return memo[diceIndex][currSum];
            }

            int ways = 0;
            for (int i = 1; i <= Math.min(k, target - currSum); i++) {
                ways = (ways + waysToTarget(memo, diceIndex + 1, n, currSum + i, target, k)) % MOD;
            }
            return memo[diceIndex][currSum] = ways;
        }

        public int numRollsToTarget1(int n, int k, int target) {
            Integer[][] memo = new Integer[n + 1][target + 1];
            return waysToTarget(memo, 0, n, 0, target, k);
        }

        public int numRollsToTarget2(int n, int k, int target) {
            int MOD = 1_000_000_007;
            int[][] dp = new int[n + 1][target + 1];
            dp[n][target] = 1;

            for (int i = n - 1; i >= 0; i--) {
                for (int j = 0; j <= target; j++) {
                    int ways = 0;

                    for (int x = 1; x <= Math.min(k, target - j); x++) {
                        ways = (ways + dp[i + 1][x + j]) % MOD;
                    }

                    dp[i][j] = ways;
                }
            }

            return dp[0][0];
        }
    }

    // https://leetcode.com/problems/maximum-length-of-pair-chain/description
    static class Solution2 {
        public int findLongestChain1(int[][] pairs) {
            var rec = new Object() {
                int longestChain(int i, int[][] pairs, int n, int[] memo) {
                    if (memo[i] != 0) {
                        return memo[i];
                    }
                    memo[i] = 1;
                    for (int j = i + 1; j < n; j++) {
                        if (pairs[i][1] < pairs[j][0]) {
                            memo[i] = Math.max(memo[i], 1 + longestChain(j, pairs, n, memo));
                        }
                    }
                    return memo[i];
                }
            };

            int n = pairs.length;
            Arrays.sort(pairs, (a, b) -> a[0] - b[0]);
            int[] memo = new int[n];

            int ans = 0;
            for (int i = 0; i < n; i++) {
                ans = Math.max(ans, rec.longestChain(i, pairs, n, memo));
            }
            return ans;
        }

        public int findLongestChain2(int[][] pairs) {
            int n = pairs.length;
            Arrays.sort(pairs, (a, b) -> a[0] - b[0]);
            int[] dp = new int[n];
            Arrays.fill(dp, 1);

            int ans = 1;
            for (int i = n - 1; i >= 0; i--) {
                for (int j = i + 1; j < n; j++) {
                    if (pairs[i][1] < pairs[j][0]) {
                        dp[i] = Math.max(dp[i], 1 + dp[j]);
                    }
                }
                ans = Math.max(ans, dp[i]);
            }

            return ans;
        }
    }

    // https://leetcode.com/problems/longest-arithmetic-subsequence-of-given-difference/description
    static class Solution3 {
        public int longestSubsequence(int[] arr, int difference) {
            Map<Integer, Integer> dp = new HashMap<>();
            int answer = 1;

            for (int a : arr) {
                int beforeA = dp.getOrDefault(a - difference, 0);
                dp.put(a, beforeA + 1);
                answer = Math.max(answer, dp.get(a));
            }

            return answer;
        }
    }

    // https://leetcode.com/problems/longest-arithmetic-subsequence/description
    static class Solution4 {
        public int longestArithSeqLength(int[] nums) {
            int maxLength = 0;
            HashMap<Integer, Integer>[] dp = new HashMap[nums.length];

            for (int right = 0; right < nums.length; ++right) {
                dp[right] = new HashMap<>();

                for (int left = 0; left < right; ++left) {
                    int diff = nums[left] - nums[right];
                    dp[right].put(diff, dp[left].getOrDefault(diff, 1) + 1);
                    maxLength = Math.max(maxLength, dp[right].get(diff));
                }
            }

            return maxLength;
        }
    }
}
