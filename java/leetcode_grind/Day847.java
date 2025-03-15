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

    // https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/description/
    static class Solution4 {
        int RADIX_1 = 26;
        int MOD_1 = 1000000033;
        int RADIX_2 = 27;
        int MOD_2 = 2147483647;

        long[] hashPair(String string, int m) {
            long hash1 = 0, hash2 = 0;
            long factor1 = 1, factor2 = 1;

            for (int i = m - 1; i >= 0; i--) {
                hash1 += ((int) (string.charAt(i) - 'a') * (factor1)) % MOD_1;
                factor1 = (factor1 * RADIX_1) % MOD_1;
                hash2 += ((int) (string.charAt(i) - 'a') * (factor2)) % MOD_2;
                factor2 = (factor2 * RADIX_2) % MOD_2;
            }

            return new long[] { hash1 % MOD_1, hash2 % MOD_2 };
        }

        public int strStr(String haystack, String needle) {
            int m = needle.length();
            int n = haystack.length();

            if (n < m)
                return -1;

            long MAX_WEIGHT_1 = 1;
            long MAX_WEIGHT_2 = 1;
            for (int i = 0; i < m; i++) {
                MAX_WEIGHT_1 = (MAX_WEIGHT_1 * RADIX_1) % MOD_1;
                MAX_WEIGHT_2 = (MAX_WEIGHT_2 * RADIX_2) % MOD_2;
            }

            long[] hashNeedle = hashPair(needle, m);
            long[] hashHay = { 0, 0 };

            for (int windowStart = 0; windowStart <= n - m; windowStart++) {
                if (windowStart == 0) {
                    hashHay = hashPair(haystack, m);
                } else {
                    hashHay[0] = (((hashHay[0] * RADIX_1) % MOD_1)
                            - (((int) (haystack.charAt(windowStart - 1) - 'a') * MAX_WEIGHT_1) % MOD_1)
                            + (int) (haystack.charAt(windowStart + m - 1) - 'a') + MOD_1) % MOD_1;
                    hashHay[1] = (((hashHay[1] * RADIX_2) % MOD_2)
                            - (((int) (haystack.charAt(windowStart - 1) - 'a') * MAX_WEIGHT_2) % MOD_2)
                            + (int) (haystack.charAt(windowStart + m - 1) - 'a') + MOD_2) % MOD_2;
                }

                if (hashNeedle[0] == hashHay[0] && hashNeedle[1] == hashHay[1]) {
                    return windowStart;
                }
            }
            return -1;
        }
    }
}
