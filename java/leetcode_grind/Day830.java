package leetcode_grind;

import java.util.HashSet;
import java.util.Set;

public class Day830 {
    // https://leetcode.com/problems/maximum-absolute-sum-of-any-subarray/description/
    static class Solution1 {
        public int maxAbsoluteSum(int[] nums) {
            int minPrefixSum = Integer.MAX_VALUE, maxPrefixSum = Integer.MIN_VALUE;
            int prefixSum = 0, maxAbsSum = 0;

            for (int i = 0; i < nums.length; i++) {
                prefixSum += nums[i];
                minPrefixSum = Math.min(minPrefixSum, prefixSum);
                maxPrefixSum = Math.max(maxPrefixSum, prefixSum);

                if (prefixSum >= 0) {
                    maxAbsSum = Math.max(maxAbsSum, Math.max(prefixSum, prefixSum - minPrefixSum));
                } else if (prefixSum <= 0) {
                    maxAbsSum = Math.max(maxAbsSum, Math.max(Math.abs(prefixSum), Math.abs(prefixSum - maxPrefixSum)));
                }
            }

            return maxAbsSum;
        }
    }

    static class Solution2 {
        public int maxAbsoluteSum(int[] nums) {
            int minPrefixSum = 0, maxPrefixSum = 0;
            int prefixSum = 0;
            for (int i = 0; i < nums.length; i++) {
                prefixSum += nums[i];
                minPrefixSum = Math.min(minPrefixSum, prefixSum);
                maxPrefixSum = Math.max(maxPrefixSum, prefixSum);
            }
            return maxPrefixSum - minPrefixSum;
        }
    }

    // https://leetcode.com/problems/longest-nice-subarray/description/
    static class Solution3 {
        public int longestNiceSubarray(int[] nums) {
            int used = 0, ans = 0;
            for (int i = 0, j = 0; i < nums.length; i++) {
                while ((nums[i] & used) != 0) {
                    used ^= nums[j++];
                }
                used |= nums[i];
                ans = Math.max(ans, i - j + 1);
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/bitwise-ors-of-subarrays/description/
    static class Solution4 {
        public int subarrayBitwiseORs(int[] arr) {
            Set<Integer> ans = new HashSet<>();
            Set<Integer> cur = new HashSet<>();
            cur.add(0);
            for (int x : arr) {
                Set<Integer> cur2 = new HashSet<>();
                for (int y : cur) {
                    cur2.add(x | y);
                }
                cur2.add(x);
                cur = cur2;
                ans.addAll(cur);
            }
            return ans.size();
        }
    }
}
