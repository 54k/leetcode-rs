package leetcode_grind;

import java.util.HashMap;

public class Day482 {
    // https://leetcode.com/problems/count-subarrays-with-more-ones-than-zeros/
    static class Solution1 {
        public int subarraysWithMoreZerosThanOnes(int[] nums) {
            int mod = 1000000007;
            var sumCount = new HashMap<Integer, Integer>();
            sumCount.put(0, 1);
            int valid = 0;
            int sum = 0;
            int total = 0;
            for (int i = 0; i < nums.length; i++) {
                if (nums[i] == 0) {
                    valid = (valid - sumCount.getOrDefault(sum - 1, 0)) % mod;
                    sum--;
                } else {
                    valid = (valid + sumCount.getOrDefault(sum, 0)) % mod;
                    sum++;
                }
                sumCount.put(sum, sumCount.getOrDefault(sum, 0) + 1);
                total = (total % mod + valid % mod) % mod;
            }
            return total;
        }
    }

    // https://leetcode.com/problems/minimum-common-value/
    static class Solution2 {
        public int getCommon(int[] nums1, int[] nums2) {
            var bs = new Object() {
                int apply(int[] a, int t) {
                    int lo = 0;
                    int hi = a.length;

                    while (lo < hi) {
                        int mid = (lo + hi) / 2;
                        if (a[mid] < t) {
                            lo = mid + 1;
                        } else {
                            hi = mid;
                        }
                    }

                    return lo;
                }
            };

            var x = 0;
            var i = 0;

            while (true) {
                if (i % 2 == 0) {
                    int y = bs.apply(nums2, nums1[x]);
                    if (y == nums2.length) {
                        return -1;
                    }
                    if (nums1[x] == nums2[y]) {
                        return nums1[x];
                    }
                    x = y;
                } else {
                    int y = bs.apply(nums1, nums2[x]);
                    if (y == nums1.length) {
                        return -1;
                    }
                    if (nums1[y] == nums2[x]) {
                        return nums1[y];
                    }
                    x = y;
                }
                i++;
            }
        }
    }
}
