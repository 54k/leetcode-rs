package leetcode_grind;
public class Day313 {
    // https://leetcode.com/problems/median-of-two-sorted-arrays/description/
    static class Solution {
        public double findMedianSortedArrays(int[] nums1, int[] nums2) {
            var solve = new Object() {
                int solve(int k, int aStart, int aEnd, int bStart, int bEnd) {
                    if (aStart > aEnd) {
                        return nums2[k - aStart];
                    } else if (bStart > bEnd) {
                        return nums1[k - bStart];
                    }

                    var aIndex = (aStart + aEnd) / 2;
                    var bIndex = (bStart + bEnd) / 2;
                    var aValue = nums1[aIndex];
                    var bValue = nums2[bIndex];

                    if (aIndex + bIndex < k) {
                        if (aValue < bValue) {
                            return solve(k, aIndex + 1, aEnd, bStart, bEnd);
                        } else {
                            return solve(k, aStart, aEnd, bIndex + 1, bEnd);
                        }
                    }

                    if (aValue < bValue) {
                        return solve(k, aStart, aEnd, bStart, bIndex - 1);
                    } else {
                        return solve(k, aStart, aIndex - 1, bStart, bEnd);
                    }
                }
            };

            var m = nums1.length;
            var n = nums2.length;
            var mn = m + n;

            if (mn % 2 == 0) {
                return (solve.solve(mn / 2, 0, m - 1, 0, n - 1) + solve.solve(mn / 2 - 1, 0, m - 1, 0, n - 1)) / 2.0;
            } else {
                return solve.solve(mn / 2, 0, m - 1, 0, n - 1);
            }
        }
    }
}
