package leetcode_grind;

import java.util.AbstractMap;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.Comparator;
import java.util.Map;
import java.util.PriorityQueue;

public class Day626 {
    // https://leetcode.com/problems/range-sum-of-sorted-subarray-sums/description/?envType=daily-question&envId=2024-08-04
    static class Solution1 {
        public int rangeSum(int[] nums, int n, int left, int right) {
            ArrayList<Integer> storeSubarray = new ArrayList<>();
            for (int i = 0; i < nums.length; i++) {
                int sum = 0;
                for (int j = i; j < nums.length; j++) {
                    sum += nums[j];
                    storeSubarray.add(sum);
                }
            }
            Collections.sort(storeSubarray);

            int rangeSum = 0, mod = (int) 1e9 + 7;
            for (int i = left - 1; i <= right - 1; i++) {
                rangeSum = (rangeSum + storeSubarray.get(i)) % mod;
            }
            return rangeSum;
        }
    }

    static class Solution2 {
        public int rangeSum(int[] nums, int n, int left, int right) {
            PriorityQueue<int[]> pq = new PriorityQueue<>(
                    new Comparator<int[]>() {
                        @Override
                        public int compare(int[] o1, int[] o2) {
                            return o1[0] - o2[0];
                        }
                    });
            for (int i = 0; i < n; i++) {
                pq.offer(new int[] { nums[i], i });
            }
            int ans = 0, mod = (int) 1e9 + 7;
            for (int i = 1; i <= right; i++) {
                int[] p = pq.poll();
                if (i >= left)
                    ans = (ans + p[0]) % mod;
                if (p[1] < n - 1) {
                    p[0] += nums[++p[1]];
                    pq.offer(p);
                }
            }
            return ans;
        }
    }

    static class Solution3 {
        private static final int MOD = (int) 1e9 + 7;

        public int rangeSum(int[] nums, int n, int left, int right) {
            long result = (sumOfFirstK(nums, n, right) - sumOfFirstK(nums, n, left - 1)) % MOD;
            return (int) ((result + MOD) % MOD);
        }

        Map.Entry<Integer, Long> countAndSum(int[] nums, int n, int target) {
            int count = 0;
            long currentSum = 0, totalSum = 0, windowSum = 0;
            for (int j = 0, i = 0; j < n; j++) {
                currentSum += nums[j];
                windowSum += nums[j] * (j - i + 1);
                while (currentSum > target) {
                    windowSum -= currentSum;
                    currentSum -= nums[i++];
                }
                count += j - i + 1;
                totalSum += windowSum;
            }
            return new AbstractMap.SimpleEntry<>(count, totalSum);
        }

        long sumOfFirstK(int[] nums, int n, int k) {
            int minSum = Arrays.stream(nums).min().getAsInt();
            int maxSum = Arrays.stream(nums).sum();
            int left = minSum, right = maxSum;

            while (left <= right) {
                int mid = left + (right - left) / 2;
                if (countAndSum(nums, n, mid).getKey() >= k)
                    right = mid - 1;
                else
                    left = mid + 1;
            }

            Map.Entry<Integer, Long> result = countAndSum(nums, n, left);
            return result.getValue() - left * (result.getKey() - k);
        }
    }
}
