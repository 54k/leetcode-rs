package leetcode_grind;

public class Day903 {
    // https://leetcode.com/problems/minimum-equal-sum-of-two-arrays-after-replacing-zeros/description/?envType=daily-question&envId=2025-05-10
    static class Solution1 {
        public long minSum(int[] nums1, int[] nums2) {
            long sum1 = 0, sum2 = 0;
            long zero1 = 0, zero2 = 0;

            for (int i : nums1) {
                sum1 += i;
                if (i == 0) {
                    sum1++;
                    zero1++;
                }
            }

            for (int i : nums2) {
                sum2 += i;
                if (i == 0) {
                    sum2++;
                    zero2++;
                }
            }

            if ((zero1 == 0 && sum2 > sum1) || (zero2 == 0 && sum1 > sum2)) {
                return -1;
            }

            return Math.max(sum1, sum2);
        }
    }
}
