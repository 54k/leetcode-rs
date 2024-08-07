package leetcode_grind;

import java.util.Arrays;
import java.util.Stack;

public class Day518 {

    // https://leetcode.com/problems/maximal-rectangle/description
    static class Solution1 {
        public int maximalRectangle(char[][] matrix) {
            if (matrix.length == 0) {
                return 0;
            }

            int m = matrix.length;
            int n = matrix[0].length;

            int[] left = new int[n];
            int[] right = new int[n];
            int[] height = new int[n];

            Arrays.fill(right, n);

            int maxarea = 0;

            for (int i = 0; i < m; i++) {
                int cur_left = 0, cur_right = n;

                for (int j = 0; j < n; j++) {
                    if (matrix[i][j] == '1') {
                        height[j]++;
                    } else {
                        height[j] = 0;
                    }
                }

                for (int j = 0; j < n; j++) {
                    if (matrix[i][j] == '1') {
                        left[j] = Math.max(left[j], cur_left);
                    } else {
                        left[j] = 0;
                        cur_left = j + 1;
                    }
                }

                for (int j = n - 1; j >= 0; j--) {
                    if (matrix[i][j] == '1') {
                        right[j] = Math.min(right[j], cur_right);
                    } else {
                        right[j] = n;
                        cur_right = j;
                    }
                }

                for (int j = 0; j < n; j++) {
                    maxarea = Math.max(maxarea, (right[j] - left[j]) * height[j]);
                }
            }
            return maxarea;
        }
    }

    static class Solution2 {
        public int leetcode84(int[] heights) {
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

}
