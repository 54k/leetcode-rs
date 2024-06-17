package leetcode_grind;

public class Day580 {
    // https://leetcode.com/problems/sum-of-square-numbers/description/
    static class Solution1 {
        public boolean judgeSquareSum(int c) {
            for (int i = 2; i * i <= c; i++) {
                int count = 0;
                if (c % i == 0) {
                    while (c % i == 0) {
                        count++;
                        c /= i;
                    }
                    if (i % 4 == 3 && count % 2 != 0) {
                        return false;
                    }
                }
            }
            return c % 4 != 3;
        }
    }

    // https://leetcode.com/problems/median-of-two-sorted-arrays/description/
    static class Solution2 {
        int p1 = 0;
        int p2 = 0;

        int getMin(int[] nums1, int[] nums2) {
            if (p1 < nums1.length && p2 < nums2.length) {
                return nums1[p1] < nums2[p2] ? nums1[p1++] : nums2[p2++];
            } else if (p1 < nums1.length) {
                return nums1[p1++];
            } else if (p2 < nums2.length) {
                return nums2[p2++];
            }
            return -1;
        }

        public double findMedianSortedArrays(int[] nums1, int[] nums2) {
            int m = nums1.length, n = nums2.length;
            if ((m + n) % 2 == 0) {
                for (int i = 0; i < (m + n) / 2 - 1; ++i) {
                    getMin(nums1, nums2);
                }
                return (double) (getMin(nums1, nums2) + getMin(nums1, nums2)) / 2;
            } else {
                for (int i = 0; i < (m + n) / 2; ++i) {
                    int tmp = getMin(nums1, nums2);
                }
                return getMin(nums1, nums2);
            }
        }
    }
}
