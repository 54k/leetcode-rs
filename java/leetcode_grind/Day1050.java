package leetcode_grind;

public class Day1050 {
    // https://leetcode.com/problems/container-with-most-water/description/?envType=daily-question&envId=2025-10-04
    static class Solution1 {
        public int maxArea(int[] height) {
            int left = 0, right = height.length - 1;
            int ans = 0;
            while (left < right) {
                ans = Math.max(ans, Math.min(height[left], height[right]) * (right - left));
                if (height[left] <= height[right]) {
                    left++;
                } else {
                    right--;
                }
            }
            return ans;
        }
    }
}
