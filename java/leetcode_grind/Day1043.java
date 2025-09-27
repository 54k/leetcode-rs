package leetcode_grind;

public class Day1043 {
    // https://leetcode.com/problems/largest-triangle-area/description/?envType=daily-question&envId=2025-09-27
    static class Solution1 {
        public double largestTriangleArea(int[][] points) {
            int N = points.length;
            double ans = 0;
            for (int i = 0; i < N; i++) {
                for (int j = i + 1; j < N; ++j) {
                    for (int k = j + 1; k < N; ++k) {
                        ans = Math.max(ans, area(points[i], points[j], points[k]));
                    }
                }
            }
            return ans;
        }

        double area(int[] P, int[] Q, int[] R) {
            return 0.5 * Math.abs(P[0] * Q[1] + Q[0] * R[1] + R[0] * P[1] - P[1] * Q[0] - Q[1] * R[0] - R[1] * P[0]);
        }
    }

    // https://leetcode.com/problems/maximum-product-subarray/description/
    static class Solution2 {
        public int maxProduct(int[] nums) {
            if (nums.length == 0)
                return 0;

            int max_so_far = nums[0];
            int min_so_far = nums[0];
            int result = max_so_far;

            for (int i = 1; i < nums.length; i++) {
                int curr = nums[i];
                int temp_max = Math.max(curr, Math.max(max_so_far * curr, min_so_far * curr));
                min_so_far = Math.min(curr, Math.min(max_so_far * curr, min_so_far * curr));
                max_so_far = temp_max;
                result = Math.max(max_so_far, result);
            }

            return result;
        }
    }
}
