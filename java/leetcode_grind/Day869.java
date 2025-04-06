package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Day869 {
    // https://leetcode.com/problems/largest-divisible-subset/submissions/1598839931/
    static class Solution1 {
        public List<Integer> largestDivisibleSubset(int[] nums) {
            int n = nums.length;
            if (n == 0)
                return new ArrayList<>();

            List<ArrayList<Integer>> EDS = new ArrayList<>();
            for (int num : nums)
                EDS.add(new ArrayList<>());

            Arrays.sort(nums);

            for (int i = 0; i < n; i++) {
                List<Integer> maxSubset = new ArrayList<>();
                for (int k = 0; k < i; k++) {
                    if (nums[i] % nums[k] == 0 && maxSubset.size() < EDS.get(k).size()) {
                        maxSubset = EDS.get(k);
                    }
                }

                EDS.get(i).addAll(maxSubset);
                EDS.get(i).add(nums[i]);
            }

            List<Integer> ret = new ArrayList<>();
            for (int i = 0; i < n; i++) {
                if (ret.size() < EDS.get(i).size())
                    ret = EDS.get(i);
            }
            return ret;
        }
    }

    static class Solution2 {
        public List<Integer> largestDivisibleSubset(int[] nums) {
            int n = nums.length;
            if (n == 0)
                return new ArrayList<>();

            Integer[] dp = new Integer[n];
            Arrays.sort(nums);

            Integer maxSubsetSize = -1, maxSubsetIndex = -1;

            for (int i = 0; i < n; i++) {
                Integer subsetSize = 0;

                for (int k = 0; k < i; k++) {
                    if (nums[i] % nums[k] == 0 && subsetSize < dp[k]) {
                        subsetSize = dp[k];
                    }
                }

                dp[i] = subsetSize + 1;

                if (maxSubsetSize < dp[i]) {
                    maxSubsetSize = dp[i];
                    maxSubsetIndex = i;
                }
            }

            LinkedList<Integer> subset = new LinkedList<>();
            Integer currSize = maxSubsetSize;
            Integer currTail = nums[maxSubsetIndex];
            for (int i = maxSubsetIndex; i >= 0; --i) {
                if (currSize == 0)
                    break;

                if (currTail % nums[i] == 0 && currSize == dp[i]) {
                    subset.addFirst(nums[i]);
                    currTail = nums[i];
                    currSize -= 1;
                }
            }
            return subset;
        }
    }
}
