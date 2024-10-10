package leetcode_grind;

import java.util.*;

public class Day694 {
    // https://leetcode.com/problems/maximum-width-ramp/description/?envType=daily-question&envId=2024-10-10
    static class Solution1 {
        public int maxWidthRamp(int[] nums) {
            int n = nums.length;
            Integer[] indices = new Integer[n];

            for (int i = 0; i < n; i++) {
                indices[i] = i;
            }

            Arrays.sort(indices, (i, j) -> nums[i] != nums[j] ? nums[i] - nums[j] : i - j);

            int minIndex = n;
            int maxWidth = 0;

            for (int i : indices) {
                maxWidth = Math.max(maxWidth, i - minIndex);
                minIndex = Math.min(minIndex, i);
            }
            return maxWidth;
        }

    }

    static class Solution2 {
        public int maxWidthRamp(int[] nums) {
            int n = nums.length;
            int[] rightMax = new int[n];
            rightMax[n - 1] = nums[n - 1];
            for (int i = n - 2; i >= 0; i--) {
                rightMax[i] = Math.max(rightMax[i + 1], nums[i]);
            }
            int left = 0, right = 0;
            int maxWidth = 0;
            while (right < n) {
                while (left < right && nums[left] > rightMax[right]) {
                    left++;
                }
                maxWidth = Math.max(maxWidth, right - left);
                right++;
            }
            return maxWidth;
        }
    }

    static class Solution3 {
        public int maxWidthRamp(int[] nums) {
            int n = nums.length;
            Stack<Integer> indicesStack = new Stack<>();
            for (int i = 0; i < n; i++) {
                if (indicesStack.isEmpty() || nums[indicesStack.peek()] > nums[i]) {
                    indicesStack.push(i);
                }
            }

            int maxWidth = 0;
            for (int j = n - 1; j >= 0; j--) {
                while (!indicesStack.isEmpty() && nums[indicesStack.peek()] <= nums[j]) {
                    maxWidth = Math.max(maxWidth, j - indicesStack.peek());
                    indicesStack.pop();
                }
            }
            return maxWidth;
        }
    }

    // https://leetcode.com/problems/minimum-time-to-build-blocks/description/
    static class Solution4 {
        public int minBuildTime(int[] blocks, int split) {
            Arrays.sort(blocks);
            for (int i = 0; i < blocks.length / 2; i++) {
                int temp = blocks[i];
                blocks[i] = blocks[blocks.length - i - 1];
                blocks[blocks.length - i - 1] = temp;
            }

            int[][] dp = new int[blocks.length][blocks.length + 1];
            for (int[] row : dp) {
                Arrays.fill(row, -1);
            }
            return solve(blocks, split, 0, 1, dp);
        }

        int solve(int[] blocks, int split, int b, int w, int[][] dp) {
            if (b == blocks.length) {
                return 0;
            }
            if (w == 0) {
                return Integer.MAX_VALUE;
            }
            if (w >= blocks.length - b) {
                return blocks[b];
            }
            if (dp[b][w] != -1) {
                return dp[b][w];
            }
            int workHere = Math.max(blocks[b], solve(blocks, split, b + 1, w - 1, dp));
            int splitHere = split + solve(blocks, split, b, Math.min(2 * w, blocks.length - b), dp);
            dp[b][w] = Math.min(workHere, splitHere);
            return dp[b][w];
        }
    }
}
