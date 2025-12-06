package leetcode_grind;

import java.util.Deque;
import java.util.LinkedList;
import java.util.TreeMap;

public class Day1113 {
    // https://leetcode.com/problems/count-partitions-with-max-min-difference-at-most-k/description/?envType=daily-question&envId=2025-12-06
    static class Solution1 {
        public int countPartitions(int[] nums, int k) {
            int n = nums.length;
            long mod = (long) 1e9 + 7;
            long[] dp = new long[n + 1];
            long[] prefix = new long[n + 1];
            TreeMap<Integer, Integer> cnt = new TreeMap<>();

            dp[0] = 1;
            prefix[0] = 1;

            for (int i = 0, j = 0; i < n; i++) {
                cnt.put(nums[i], cnt.getOrDefault(nums[i], 0) + 1);
                while (j <= i && cnt.lastKey() - cnt.firstKey() > k) {
                    cnt.put(nums[j], cnt.get(nums[j]) - 1);
                    if (cnt.get(nums[j]) == 0) {
                        cnt.remove(nums[j]);
                    }
                    j++;
                }

                dp[i + 1] = (prefix[i] - (j > 0 ? prefix[j - 1] : 0) + mod) % mod;
                prefix[i + 1] = (prefix[i] + dp[i + 1]) % mod;
            }

            return (int) dp[n];
        }
    }

    static class Solution2 {

        public int countPartitions(int[] nums, int k) {
            int n = nums.length;
            long mod = (long) 1e9 + 7;
            long[] dp = new long[n + 1];
            long[] prefix = new long[n + 1];
            Deque<Integer> minQ = new LinkedList<>();
            Deque<Integer> maxQ = new LinkedList<>();

            dp[0] = 1;
            prefix[0] = 1;
            for (int i = 0, j = 0; i < n; i++) {
                // maintain the maximum value queue
                while (!maxQ.isEmpty() && nums[maxQ.peekLast()] <= nums[i]) {
                    maxQ.pollLast();
                }
                maxQ.offerLast(i);
                // maintain the minimum value queue
                while (!minQ.isEmpty() && nums[minQ.peekLast()] >= nums[i]) {
                    minQ.pollLast();
                }
                minQ.offerLast(i);
                // adjust window
                while (!maxQ.isEmpty() &&
                        !minQ.isEmpty() &&
                        nums[maxQ.peekFirst()] - nums[minQ.peekFirst()] > k) {
                    if (maxQ.peekFirst() == j) {
                        maxQ.pollFirst();
                    }
                    if (minQ.peekFirst() == j) {
                        minQ.pollFirst();
                    }
                    j++;
                }

                dp[i + 1] = (prefix[i] - (j > 0 ? prefix[j - 1] : 0) + mod) % mod;
                prefix[i + 1] = (prefix[i] + dp[i + 1]) % mod;
            }

            return (int) dp[n];
        }
    }
}
