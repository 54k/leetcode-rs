package leetcode_grind;

public class Day624 {
    // https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together-ii/description/?envType=daily-question&envId=2024-08-02
    static class Solution1 {
        public int minSwaps(int[] nums) {
            int total = 0;
            ;
            for (int i = 0; i < nums.length; i++) {
                total += nums[i];
            }
            int j = 0;
            int curr = 0;
            int ans = Integer.MAX_VALUE;
            for (int i = 0; i < nums.length * 2; i++) {
                curr += nums[i % nums.length];
                if (i >= total) {
                    curr -= nums[j % nums.length];
                    j++;
                }
                ans = Math.min(ans, total - curr);
            }
            return ans;
        }
    }

    static class Solution2 {
        public int minSwaps(int[] nums) {
            int op1 = minSwapHelper(nums, 0);
            int op2 = minSwapHelper(nums, 1);
            return Math.min(op1, op2);
        }

        int minSwapHelper(int[] data, int val) {
            int length = data.length;
            int[] rightSuffixSum = new int[length + 1];
            for (int i = length - 1; i >= 0; i--) {
                rightSuffixSum[i] = rightSuffixSum[i + 1];
                if (data[i] == (val ^ 1))
                    rightSuffixSum[i]++;
            }
            int totalSwapsNeeded = rightSuffixSum[0];
            int currentSwapCount = 0;
            int minimumSwaps = totalSwapsNeeded - rightSuffixSum[length - totalSwapsNeeded];

            for (int i = 0; i < totalSwapsNeeded; i++) {
                if (data[i] == (val ^ 1))
                    currentSwapCount++;
                int remaining = (totalSwapsNeeded - i - 1);
                int requiredSwaps = ((i + 1) - currentSwapCount) + (remaining - rightSuffixSum[length - remaining]);
                minimumSwaps = Math.min(minimumSwaps, requiredSwaps);
            }
            return minimumSwaps;
        }
    }

    static class Solution3 {
        public int minSwaps(int[] nums) {
            int op1 = minSwapHelper(nums, 0);
            int op2 = minSwapHelper(nums, 1);
            return Math.min(op1, op2);
        }

        int minSwapHelper(int[] data, int val) {
            int length = data.length;
            int totalValCount = 0;

            for (int i = length - 1; i >= 0; i--) {
                if (data[i] == val)
                    totalValCount++;
            }

            if (totalValCount == 0 || totalValCount == length)
                return 0;

            int start = 0, end = 0;
            int maxValInWindow = 0, currentValInWindow = 0;

            while (end < totalValCount) {
                if (data[end++] == val)
                    currentValInWindow++;
            }
            maxValInWindow = Math.max(maxValInWindow, currentValInWindow);

            while (end < length) {
                if (data[start++] == val)
                    currentValInWindow--;
                if (data[end++] == val)
                    currentValInWindow++;
                maxValInWindow = Math.max(maxValInWindow, currentValInWindow);
            }
            return totalValCount - maxValInWindow;
        }
    }
}
