package leetcode_grind;

import java.util.ArrayDeque;
import java.util.Deque;

public class Day572 {
    // https://leetcode.com/problems/subarray-sums-divisible-by-k/description
    static class Solution1 {
        public int subarraysDivByK(int[] nums, int k) {
            var modGroup = new int[k];
            modGroup[0] = 1;
            var ans = 0;
            var sum = 0;
            for (int i = 0; i < nums.length; i++) {
                sum += (nums[i] % k + k) % k;
                sum %= k;
                ans += modGroup[sum];
                modGroup[sum]++;

            }
            return ans;
        }
    }

    // https://leetcode.com/problems/largest-rectangle-in-histogram/
    static class Solution2 {
        public int largestRectangleArea(int[] heights) {
            Deque<Integer> stack = new ArrayDeque<>();
            stack.push(-1);
            int n = heights.length;
            int maxArea = 0;
            for (int i = 0; i <= n; ++i) {
                while ((stack.peek() != -1) && (i == n || heights[stack.peek()] >= heights[i])) {
                    int currentHeight = heights[stack.pop()];
                    int currentWidth = i - stack.peek() - 1;
                    maxArea = Math.max(maxArea, currentHeight * currentWidth);
                }
                stack.push(i);
            }
            return maxArea;
        }
    }
}
