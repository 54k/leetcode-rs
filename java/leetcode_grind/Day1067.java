package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

public class Day1067 {
    // https://leetcode.com/problems/maximum-frequency-of-an-element-after-performing-operations-i/description/?envType=daily-question&envId=2025-10-21
    static class Solution1 {
        public int maxFrequency(int[] nums, int k, int numOperations) {
            int n = nums.length, ans = 0, left = 0, right = 0;
            Arrays.sort(nums);

            Map<Integer, Integer> count = new HashMap<>();
            for (int num : nums)
                count.put(num, count.getOrDefault(num, 0) + 1);

            for (int mid = 0; mid < n; mid++) {
                while (nums[mid] - nums[left] > k) {
                    left++;
                }

                while (right < n - 1 && nums[right + 1] - nums[mid] <= k) {
                    right++;
                }

                int total = right - left + 1;
                ans = Math.max(ans, Math.min(total - count.get(nums[mid]), numOperations) + count.get(nums[mid]));
            }

            left = 0;
            for (right = 0; right < n; right++) {
                int mid = (nums[left] + nums[right]) / 2;
                while (mid - nums[left] > k || nums[right] - mid > k) {
                    left++;
                    mid = (nums[left] + nums[right]) / 2;
                }
                ans = Math.max(ans, Math.min(right - left + 1, numOperations));
            }
            return ans;
        }
    }

    static class Solution2 {

        public int maxFrequency(int[] nums, int k, int numOperations) {
            Arrays.sort(nums);

            int ans = 0;
            Map<Integer, Integer> numCount = new HashMap<>();

            int lastNumIndex = 0;
            for (int i = 0; i < nums.length; ++i) {
                if (nums[i] != nums[lastNumIndex]) {
                    numCount.put(nums[lastNumIndex], i - lastNumIndex);
                    ans = Math.max(ans, i - lastNumIndex);
                    lastNumIndex = i;
                }
            }

            numCount.put(nums[lastNumIndex], nums.length - lastNumIndex);
            ans = Math.max(ans, nums.length - lastNumIndex);

            for (int i = nums[0]; i <= nums[nums.length - 1]; i++) {
                int l = leftBound(nums, i - k);
                int r = rightBound(nums, i + k);
                int tempAns;
                if (numCount.containsKey(i)) {
                    tempAns = Math.min(r - l + 1, numCount.get(i) + numOperations);
                } else {
                    tempAns = Math.min(r - l + 1, numOperations);
                }
                ans = Math.max(ans, tempAns);
            }

            return ans;
        }

        private int leftBound(int[] nums, int value) {
            int left = 0;
            int right = nums.length - 1;
            while (left < right) {
                int mid = (left + right) / 2;
                if (nums[mid] < value) {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            return left;
        }

        private int rightBound(int[] nums, int value) {
            int left = 0;
            int right = nums.length - 1;
            while (left < right) {
                int mid = (left + right + 1) / 2;
                if (nums[mid] > value) {
                    right = mid - 1;
                } else {
                    left = mid;
                }
            }
            return left;
        }
    }
}
