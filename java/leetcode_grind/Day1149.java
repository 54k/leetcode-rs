package leetcode_grind;

import java.util.ArrayDeque;
import java.util.Deque;
import java.util.Stack;

public class Day1149 {
    // https://leetcode.com/problems/maximal-rectangle/description/?envType=daily-question&envId=2026-01-11
    static class Solution1 {
        public int maximalRectangle(char[][] matrix) {
            if (matrix.length == 0)
                return 0;
            int maxarea = 0;
            int[][] dp = new int[matrix.length][matrix[0].length];

            for (int i = 0; i < matrix.length; i++) {
                for (int j = 0; j < matrix[0].length; j++) {
                    if (matrix[i][j] == '1') {
                        dp[i][j] = j == 0 ? 1 : dp[i][j - 1] + 1;

                        int width = dp[i][j];

                        for (int k = i; k >= 0; k--) {
                            width = Math.min(width, dp[k][j]);
                            maxarea = Math.max(maxarea, width * (i - k + 1));
                        }
                    }
                }
            }

            return maxarea;
        }
    }

    static class Solution2 {
        int leetcode84(int[] heights) {
            Stack<Integer> stack = new Stack<>();
            stack.push(-1);
            int maxarea = 0;
            for (int i = 0; i < heights.length; i++) {
                while (stack.peek() != -1 && heights[stack.peek()] >= heights[i]) {
                    maxarea = Math.max(maxarea, heights[stack.pop()] * (i - stack.peek() - 1));
                }
                stack.push(i);
            }

            while (stack.peek() != -1) {
                maxarea = Math.max(maxarea, heights[stack.pop()] * (heights.length - stack.peek() - 1));
            }

            return maxarea;
        }

        public int maximalRectangle(char[][] matrix) {
            if (matrix.length == 0)
                return 0;
            int maxarea = 0;
            int[] dp = new int[matrix[0].length];
            for (int i = 0; i < matrix.length; i++) {
                for (int j = 0; j < matrix[0].length; j++) {
                    dp[j] = matrix[i][j] == '1' ? dp[j] + 1 : 0;
                }

                maxarea = Math.max(maxarea, leetcode84(dp));
            }
            return maxarea;
        }
    }

    // https://leetcode.com/problems/largest-rectangle-in-histogram/description/
    static class Solution3 {
        public int largestRectangleArea(int[] heights) {
            Deque<Integer> stack = new ArrayDeque<>();
            stack.push(-1);
            int length = heights.length;
            int maxArea = 0;
            for (int i = 0; i < length; i++) {
                while (stack.peek() != -1 && heights[stack.peek()] >= heights[i]) {
                    int currentHeight = heights[stack.pop()];
                    int currentWidth = i - stack.peek() - 1;
                    maxArea = Math.max(maxArea, currentHeight * currentWidth);
                }
                stack.push(i);
            }

            while (stack.peek() != -1) {
                int currentHeight = heights[stack.pop()];
                int currentWidth = length - stack.peek() - 1;
                maxArea = Math.max(maxArea, currentHeight * currentWidth);
            }

            return maxArea;
        }
    }
}
