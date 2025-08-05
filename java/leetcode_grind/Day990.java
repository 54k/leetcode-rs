package leetcode_grind;

import java.util.Arrays;

public class Day990 {

    // https://leetcode.com/problems/fruits-into-baskets-ii/description/
    static class Solution1 {
        public int numOfUnplacedFruits(int[] fruits, int[] baskets) {
            int count = 0;
            int n = baskets.length;

            for (int fruit : fruits) {
                int unset = 1;
                for (int i = 0; i < n; i++) {
                    if (fruit <= baskets[i]) {
                        baskets[i] = 0;
                        unset = 0;
                        break;
                    }
                }
                count += unset;
            }

            return count;
        }
    }

    // https://leetcode.com/problems/kth-smallest-subarray-sum/description/
    static class Solution2 {
        int n;
        int[] nums;

        int numberOfSubarraySumLessThanX(int x) {
            int cnt = 0, cur = 0, j = 0;
            for (int i = 0; i < n; i++) {
                cur += nums[i];
                while (x < cur) {
                    cur -= nums[j];
                    j += 1;
                }
                cnt += i - j + 1;
            }
            return cnt;
        }

        public int kthSmallestSubarraySum(int[] nums, int k) {
            this.nums = nums;
            this.n = nums.length;
            int low = 0, high = Arrays.stream(nums).sum(), mid;
            while (low <= high) {
                mid = low + (high - low) / 2;
                if (k <= numberOfSubarraySumLessThanX(mid)) {
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            }
            return low;
        }
    }

}
