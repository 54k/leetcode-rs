package leetcode_grind;

public class Day982 {
    // https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/description/?envType=daily-question&envId=2025-07-28
    static class Solution1 {
        public int countMaxOrSubsets(int[] nums) {
            int maxOrValue = 0;
            for (int num : nums) {
                maxOrValue |= num;
            }
            return countSubsets(nums, 0, 0, maxOrValue);
        }

        int countSubsets(int[] nums, int index, int currentOr, int targetOr) {
            if (index == nums.length) {
                return (currentOr == targetOr) ? 1 : 0;
            }

            int countWithout = countSubsets(nums, index + 1, currentOr, targetOr);
            int countWith = countSubsets(nums, index + 1, currentOr | nums[index], targetOr);

            return countWith + countWithout;
        }
    }

    static class Solution2 {
        public int countMaxOrSubsets(int[] nums) {
            int n = nums.length;
            int maxOrValue = 0;
            for (int num : nums) {
                maxOrValue |= num;
            }

            Integer[][] memo = new Integer[n][maxOrValue + 1];
            return countSubsets(nums, 0, 0, maxOrValue, memo);
        }

        int countSubsets(int[] nums, int index, int currentOr, int targetOr, Integer[][] memo) {
            if (index == nums.length) {
                return (currentOr == targetOr) ? 1 : 0;
            }

            if (memo[index][currentOr] != null) {
                return memo[index][currentOr];
            }

            int countWithout = countSubsets(nums, index + 1, currentOr, targetOr, memo);
            int countWith = countSubsets(nums, index + 1, currentOr | nums[index], targetOr, memo);

            return memo[index][currentOr] = countWith + countWithout;
        }
    }

    static class Solution3 {
        public int countMaxOrSubsets(int[] nums) {
            int maxOrValue = 0;
            for (int num : nums) {
                maxOrValue |= num;
            }

            int totalSubsets = 1 << nums.length;
            int subsetWithMaxOr = 0;

            for (int subsetMask = 0; subsetMask < totalSubsets; subsetMask++) {
                int currentOrValue = 0;
                for (int i = 0; i < nums.length; i++) {
                    if (((subsetMask >> i) & 1) == 1) {
                        currentOrValue |= nums[i];
                    }
                }
                if (currentOrValue == maxOrValue) {
                    subsetWithMaxOr++;
                }
            }

            return subsetWithMaxOr;
        }
    }

    static class Solution4 {
        public int countMaxOrSubsets(int[] nums) {
            int max = 0;
            int[] dp = new int[1 << 17];
            dp[0] = 1;

            for (int num : nums) {
                for (int i = max; i >= 0; i--) {
                    dp[i | num] += dp[i];
                }
                max |= num;
            }

            return dp[max];
        }
    }
}
