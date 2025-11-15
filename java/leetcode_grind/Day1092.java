package leetcode_grind;

import java.util.Arrays;

public class Day1092 {
    // https://leetcode.com/problems/3sum-smaller/description/?envType=weekly-question&envId=2025-11-15
    static class Solution1 {
        public int threeSumSmaller(int[] nums, int target) {
            Arrays.sort(nums);
            int sum = 0;
            for (int i = 0; i < nums.length - 2; i++) {
                sum += twoSumSmaller(nums, i + 1, target - nums[i]);
            }
            return sum;
        }

        int twoSumSmaller(int[] nums, int startIndex, int target) {
            int sum = 0;
            for (int i = startIndex; i < nums.length - 1; i++) {
                int j = binarySearch(nums, i, target - nums[i]);
                sum += j - i;
            }
            return sum;
        }

        int binarySearch(int[] nums, int startIndex, int target) {
            int left = startIndex;
            int right = nums.length - 1;
            while (left < right) {
                int mid = (left + right + 1) / 2;
                if (nums[mid] < target) {
                    left = mid;
                } else {
                    right = mid - 1;
                }
            }
            return left;
        }
    }

    static class Solution2 {
        public int threeSumSmaller(int[] nums, int target) {
            Arrays.sort(nums);
            int sum = 0;
            for (int i = 0; i < nums.length - 2; i++) {
                sum += twoSumSmaller(nums, i + 1, target - nums[i]);
            }
            return sum;
        }

        int twoSumSmaller(int[] nums, int startIndex, int target) {
            int sum = 0;
            int left = startIndex;
            int right = nums.length - 1;
            while (left < right) {
                if (nums[left] + nums[right] < target) {
                    sum += right - left;
                    left++;
                } else {
                    right--;
                }
            }
            return sum;
        }
    }

    // https://leetcode.com/problems/count-the-number-of-substrings-with-dominant-ones/description/?envType=daily-question&envId=2025-11-15
    static class Solution3 {
        public int numberOfSubstrings(String s) {
            int n = s.length();
            int[] pre = new int[n + 1];
            pre[0] = -1;
            for (int i = 0; i < n; i++) {
                if (i == 0 || (i > 0 && s.charAt(i - 1) == '0')) {
                    pre[i + 1] = i;
                } else {
                    pre[i + 1] = pre[i];
                }
            }
            int res = 0;
            for (int i = 1; i <= n; i++) {
                int cnt0 = s.charAt(i - 1) == '0' ? 1 : 0;
                int j = i;
                while (j > 0 && cnt0 * cnt0 <= n) {
                    int cnt1 = (i - pre[j]) - cnt0;
                    if (cnt0 * cnt0 <= cnt1) {
                        res += Math.min(j - pre[j], cnt1 - cnt0 * cnt0 + 1);
                    }
                    j = pre[j];
                    cnt0++;
                }
            }
            return res;
        }
    }

}
