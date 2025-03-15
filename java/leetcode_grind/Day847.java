package leetcode_grind;

import java.util.Arrays;

public class Day847 {
    // https://leetcode.com/problems/container-with-most-water/description/
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

    // https://leetcode.com/problems/palindrome-permutation/description/?envType=weekly-question&envId=2025-03-15
    static class Solution2 {
        public boolean canPermutePalindrome(String s) {
            int[] map = new int[128];
            int count = 0;
            for (int i = 0; i < s.length(); i++) {
                map[s.charAt(i)]++;
                if (map[s.charAt(i)] % 2 == 0)
                    count--;
                else
                    count++;
            }
            return count <= 1;
        }
    }

    // https://leetcode.com/problems/house-robber-iv/description/?envType=daily-question&envId=2025-03-15
    static class Solution3 {
        public int minCapability(int[] nums, int k) {
            int minReward = 1;
            int maxReward = Arrays.stream(nums).max().getAsInt();
            int totalHouses = nums.length;

            while (minReward < maxReward) {
                int midReward = (minReward + maxReward) / 2;
                int possibleTheft = 0;

                for (int index = 0; index < totalHouses; ++index) {
                    if (nums[index] <= midReward) {
                        possibleTheft += 1;
                        index++;
                    }
                }

                if (possibleTheft >= k)
                    maxReward = midReward;
                else
                    minReward = midReward + 1;
            }

            return minReward;
        }
    }

}
