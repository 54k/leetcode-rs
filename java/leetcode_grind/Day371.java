package leetcode_grind;

import java.util.Arrays;

public class Day371 {
    // https://leetcode.com/problems/frequency-of-the-most-frequent-element/description
    static class Solution1 {
        public int maxFrequency1(int[] nums, int k) {
            Arrays.sort(nums);
            int ans = 0;
            int left = 0;
            int running = 0;
            for (int right = 0; right < nums.length; right++) {
                running += nums[right];
                while (nums[right] * (right - left + 1) - running > k) {
                    running -= nums[left++];
                }
                ans = Math.max(ans, right - left + 1);
            }
            return ans;
        }

        public int maxFrequency2(int[] nums, int k) {
            Arrays.sort(nums);
            int left = 0;
            int running = 0;

            for (int right = 0; right < nums.length; right++) {
                running += nums[right];

                if (nums[right] * (right - left + 1) - running > k) {
                    running -= nums[left++];
                }
            }
            return nums.length - left;
        }

        public int maxFrequency3(int[] nums, int k) {
            Arrays.sort(nums);
            int ans = 0;

            long[] prefix = new long[nums.length];
            prefix[0] = nums[0];
            for (int i = 1; i < prefix.length; i++) {
                prefix[i] = prefix[i - 1] + nums[i];
            }

            for (int i = 0; i < nums.length; i++) {
                int target = nums[i];

                int lo = 0;
                int hi = i;
                int best = i;

                while (lo <= hi) {
                    int mid = (lo + hi) / 2;

                    long count = i - mid + 1;
                    long finalSum = count * target;
                    long originalSum = prefix[i] - prefix[mid] + nums[mid]; // fancy variant for prefix[0 - 1] case
                    long operationsRequired = finalSum - originalSum;

                    if (operationsRequired > k) {
                        lo = mid + 1;
                    } else {
                        best = mid;
                        hi = mid - 1;
                    }
                }

                ans = Math.max(ans, i - best + 1);
            }
            return ans;
        }
    }

}
