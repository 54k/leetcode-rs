package leetcode_grind;

import java.util.Arrays;
import java.util.HashSet;
import java.util.Set;

public class Day849 {
    // https://leetcode.com/problems/divide-array-into-equal-pairs/description/?envType=daily-question&envId=2025-03-17
    static class Solution1 {
        public boolean divideArray(int[] nums) {
            Arrays.sort(nums);
            for (int pos = 0; pos < nums.length; pos += 2) {
                if (nums[pos] != nums[pos + 1]) {
                    return false;
                }
            }
            return true;
        }
    }

    static class Solution2 {
        public boolean divideArray(int[] nums) {
            Set<Integer> unpaired = new HashSet<>();
            for (int num : nums) {
                if (unpaired.contains(num)) {
                    unpaired.remove(num);
                } else {
                    unpaired.add(num);
                }
            }
            return unpaired.isEmpty();
        }
    }

    // https://leetcode.com/problems/split-array-largest-sum/description/
    static class Solution3 {
        int[][] memo = new int[1001][51];

        public int splitArray(int[] nums, int k) {
            int n = nums.length;
            int[] prefixSum = new int[n + 1];
            for (int i = 0; i < n; i++) {
                prefixSum[i + 1] = prefixSum[i] + nums[i];
            }

            for (int subarrayCount = 1; subarrayCount <= k; subarrayCount++) {
                for (int currIndex = 0; currIndex < n; currIndex++) {
                    if (subarrayCount == 1) {
                        memo[currIndex][subarrayCount] = prefixSum[n] - prefixSum[currIndex];
                        continue;
                    }

                    int minimumLargestSplitSum = Integer.MAX_VALUE;
                    for (int i = currIndex; i <= n - subarrayCount; i++) {
                        int firstSplitSum = prefixSum[i + 1] - prefixSum[currIndex];
                        int largestSplitSum = Math.max(firstSplitSum, memo[i + 1][subarrayCount - 1]);

                        minimumLargestSplitSum = Math.min(minimumLargestSplitSum, largestSplitSum);

                        if (firstSplitSum >= minimumLargestSplitSum) {
                            break;
                        }
                    }

                    memo[currIndex][subarrayCount] = minimumLargestSplitSum;
                }
            }

            return memo[0][k];
        }
    }
}
