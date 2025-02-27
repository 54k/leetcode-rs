package leetcode_grind;

import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;

public class Day831 {
    // https://leetcode.com/problems/length-of-longest-fibonacci-subsequence/description/?envType=daily-question&envId=2025-02-27
    static class Solution1 {
        public int lenLongestFibSubseq(int[] arr) {
            int n = arr.length;

            Set<Integer> numSet = new HashSet<>();
            for (int num : arr) {
                numSet.add(num);
            }

            int maxLen = 0;
            for (int start = 0; start < n; start++) {
                for (int next = start + 1; next < n; next++) {
                    int prev = arr[next];
                    int curr = arr[start] + arr[next];
                    int len = 2;

                    while (numSet.contains(curr)) {
                        int temp = curr;
                        curr += prev;
                        prev = temp;
                        maxLen = Math.max(maxLen, ++len);
                    }
                }
            }

            return maxLen;
        }
    }

    static class Solution2 {
        public int lenLongestFibSubseq(int[] arr) {
            int maxLen = 0;

            int[][] dp = new int[arr.length][arr.length];
            Map<Integer, Integer> valToIdx = new HashMap<>();

            for (int curr = 0; curr < arr.length; curr++) {
                valToIdx.put(arr[curr], curr);

                for (int prev = 0; prev < curr; prev++) {
                    int diff = arr[curr] - arr[prev];
                    int prevIdx = valToIdx.getOrDefault(diff, -1);

                    if (diff < arr[prev] && prevIdx >= 0) {
                        dp[prev][curr] = dp[prevIdx][prev] + 1;
                    } else {
                        dp[prev][curr] = 2;
                    }

                    maxLen = Math.max(maxLen, dp[prev][curr]);
                }
            }

            return maxLen > 2 ? maxLen : 0;
        }
    }

    static class Solution3 {
        public int lenLongestFibSubseq(int[] arr) {
            int n = arr.length;
            int[][] dp = new int[n][n];
            int maxLen = 0;

            for (int curr = 2; curr < n; curr++) {
                int start = 0;
                int end = curr - 1;

                while (start < end) {
                    int pairSum = arr[start] + arr[end];

                    if (pairSum > arr[curr]) {
                        end--;
                    } else if (pairSum < arr[curr]) {
                        start++;
                    } else {
                        dp[end][curr] = dp[start][end] + 1;
                        maxLen = Math.max(dp[end][curr], maxLen);
                        end--;
                        start++;
                    }
                }
            }

            return maxLen == 0 ? 0 : maxLen + 2;
        }
    }

}
