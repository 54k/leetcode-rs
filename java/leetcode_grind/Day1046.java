package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;

public class Day1046 {
    // https://leetcode.com/problems/find-triangular-sum-of-an-array/description/?envType=daily-question&envId=2025-09-30
    static class Solution1 {
        public int triangularSum(int[] nums) {
            var lst = Arrays.stream(nums).boxed().toList();
            while (lst.size() > 1) {
                var newLst = new ArrayList<Integer>();
                for (int i = 0; i < lst.size() - 1; i++) {
                    newLst.add((lst.get(i) + lst.get(i + 1)) % 10);
                }
                lst = newLst;
            }
            return lst.get(0);
        }
    }

    // https://leetcode.com/problems/split-array-largest-sum/description/
    static class Solution2 {

        Integer[][] memo = new Integer[1001][51];

        int getMinimumLargestSplitSum(int[] prefixSum, int currIndex, int subarrayCount) {
            int n = prefixSum.length - 1;

            if (memo[currIndex][subarrayCount] != null) {
                return memo[currIndex][subarrayCount];
            }

            if (subarrayCount == 1) {
                return memo[currIndex][subarrayCount] = prefixSum[n] - prefixSum[currIndex];
            }

            int minimumLargestSplitSum = Integer.MAX_VALUE;

            for (int i = currIndex; i <= n - subarrayCount; i++) {
                int firstSplitSum = prefixSum[i + 1] - prefixSum[currIndex];

                int largestSplitSum = Math.max(firstSplitSum,
                        getMinimumLargestSplitSum(prefixSum, i + 1, subarrayCount - 1));

                minimumLargestSplitSum = Math.min(minimumLargestSplitSum, largestSplitSum);

                if (firstSplitSum >= minimumLargestSplitSum) {
                    break;
                }

            }

            return memo[currIndex][subarrayCount] = minimumLargestSplitSum;
        }

        public int splitArray(int[] nums, int k) {
            int n = nums.length;
            int[] prefixSum = new int[n + 1];

            for (int i = 0; i < n; i++) {
                prefixSum[i + 1] = prefixSum[i] + nums[i];
            }

            return getMinimumLargestSplitSum(prefixSum, 0, k);
        }
    }

    static class Solution3 {

        Integer[][] memo = new Integer[1001][51];

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
