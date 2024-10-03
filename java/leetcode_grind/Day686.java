package leetcode_grind;

import java.util.*;

public class Day686 {
    // https://leetcode.com/problems/make-sum-divisible-by-p/description/?envType=daily-question&envId=2024-10-03
    static class Solution1 {
        public int minSubarray(int[] nums, int p) {
            int n = nums.length;
            long totalSum = 0;
            for (int num : nums) {
                totalSum += num;
            }

            if (totalSum % p == 0)
                return 0;
            int minLen = n;

            for (int start = 0; start < n; start++) {
                long subSum = 0;
                for (int end = start; end < n; end++) {
                    subSum += nums[end];
                    long remainingSum = (totalSum - subSum) % p;
                    if (remainingSum == 0) {
                        minLen = Math.min(minLen, end - start + 1);
                    }
                }
            }
            return minLen == n ? -1 : minLen;
        }
    }

    static class Solution2 {
        public int minSubarray(int[] nums, int p) {
            int n = nums.length;
            int totalSum = 0;
            for (int num : nums) {
                totalSum = (totalSum + num) % p;
            }
            int target = totalSum % p;
            if (target == 0) {
                return 0;
            }
            HashMap<Integer, Integer> modMap = new HashMap<>();
            modMap.put(0, -1);
            int currentSum = 0;
            int minLen = n;
            for (int i = 0; i < n; i++) {
                currentSum = (currentSum + nums[i]) % p;
                int needed = (currentSum - target + p) % p;
                if (modMap.containsKey(needed)) {
                    minLen = Math.min(minLen, i - modMap.get(needed));
                }
                modMap.put(currentSum, i);
            }
            return minLen == n ? -1 : minLen;
        }
    }

}
