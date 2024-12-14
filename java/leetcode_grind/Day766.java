package leetcode_grind;

import java.util.ArrayDeque;
import java.util.Arrays;
import java.util.Deque;
import java.util.PriorityQueue;
import java.util.TreeMap;

public class Day766 {
    // https://leetcode.com/problems/continuous-subarrays/
    static class Solution1 {
        public long continuousSubarrays(int[] nums) {
            Deque<Integer> maxQ = new ArrayDeque<>();
            Deque<Integer> minQ = new ArrayDeque<>();
            int left = 0;
            long count = 0;

            for (int right = 0; right < nums.length; right++) {
                while (!maxQ.isEmpty() && nums[maxQ.peekLast()] < nums[right]) {
                    maxQ.pollLast();
                }
                maxQ.offerLast(right);

                while (!minQ.isEmpty() && nums[minQ.peekLast()] > nums[right]) {
                    minQ.pollLast();
                }
                minQ.offerLast(right);

                while (!maxQ.isEmpty() && !minQ.isEmpty() && nums[maxQ.peekFirst()] - nums[minQ.peekFirst()] > 2) {
                    if (maxQ.peekFirst() < minQ.peekFirst()) {
                        left = maxQ.peekFirst() + 1;
                        maxQ.pollFirst();
                    } else {
                        left = minQ.peekFirst() + 1;
                        minQ.pollFirst();
                    }
                }

                count += right - left + 1;
            }
            return count;
        }
    }

    static class Solution2 {
        public long continuousSubarrays(int[] nums) {
            int left = 0, right = 0;
            long count = 0;

            PriorityQueue<Integer> minHeap = new PriorityQueue<>(
                    (a, b) -> nums[a] - nums[b]);
            PriorityQueue<Integer> maxHeap = new PriorityQueue<>(
                    (a, b) -> nums[b] - nums[a]);

            while (right < nums.length) {
                minHeap.add(right);
                maxHeap.add(right);

                while (left < right && nums[maxHeap.peek()] - nums[minHeap.peek()] > 2) {
                    left++;

                    while (!maxHeap.isEmpty() && maxHeap.peek() < left) {
                        maxHeap.poll();
                    }
                    while (!minHeap.isEmpty() && minHeap.peek() < left) {
                        minHeap.poll();
                    }
                }

                count += right - left + 1;
                right++;
            }

            return count;
        }
    }

    static class Solution3 {
        public long continuousSubarrays(int[] nums) {
            TreeMap<Integer, Integer> freq = new TreeMap<>();
            int left = 0, right = 0;
            int n = nums.length;
            long count = 0;

            while (right < n) {
                freq.put(nums[right], freq.getOrDefault(nums[right], 0) + 1);

                while (freq.lastEntry().getKey() - freq.firstEntry().getKey() > 2) {
                    freq.put(nums[left], freq.get(nums[left]) - 1);
                    if (freq.get(nums[left]) == 0) {
                        freq.remove(nums[left]);
                    }
                    left++;
                }

                count += right - left + 1;
                right++;
            }

            return count;
        }
    }

    // https://leetcode.com/problems/pizza-with-3n-slices/
    static class Solution4 {
        public int maxSizeSlices(int[] slices) {
            int m = slices.length, n = m / 3;
            int[] slices1 = Arrays.copyOfRange(slices, 0, m - 1);
            int[] slices2 = Arrays.copyOfRange(slices, 1, m);
            return Math.max(maxSum(slices1, n), maxSum(slices2, n));
        }

        int maxSum(int[] arr, int n) {
            int m = arr.length;
            int[][] dp = new int[m + 1][n + 1];
            for (int i = 1; i <= m; i++) {
                for (int j = 1; j <= n; j++) {
                    if (i == 1) {
                        dp[i][j] = arr[0];
                    } else {
                        dp[i][j] = Math.max(dp[i - 1][j], dp[i - 2][j - 1] + arr[i - 1]);
                    }
                }
            }
            return dp[m][n];
        }
    }
}
