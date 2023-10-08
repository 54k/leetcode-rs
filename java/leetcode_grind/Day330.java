package leetcode_grind;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Consumer;

public class Day330 {
    // https://leetcode.com/problems/max-dot-product-of-two-subsequences/description/
    static class Solution1 {
        public int maxDotProduct(int[] nums1, int[] nums2) {
            var maxNum1 = Integer.MIN_VALUE;
            var minNum1 = Integer.MAX_VALUE;
            for (var n : nums1) {
                maxNum1 = Math.max(maxNum1, n);
                minNum1 = Math.min(minNum1, n);
            }

            var maxNum2 = Integer.MIN_VALUE;
            var minNum2 = Integer.MAX_VALUE;
            for (var n : nums2) {
                maxNum2 = Math.max(maxNum2, n);
                minNum2 = Math.min(minNum2, n);
            }

            if (maxNum1 < 0 && minNum2 > 0) {
                return maxNum1 * minNum2;
            }
            if (minNum1 > 0 && maxNum2 < 0) {
                return minNum1 * maxNum2;
            }

            var m = nums2.length + 1;
            var prevDp = new int[m];
            var dp = new int[m];

            for (int i = nums1.length - 1; i >= 0; i--) {
                dp = new int[m];
                for (int j = nums2.length - 1; j >= 0; j--) {
                    var use = nums1[i] * nums2[j] + prevDp[j + 1];
                    dp[j] = Math.max(use, Math.max(prevDp[j], dp[j + 1]));
                }
                prevDp = dp;
            }

            return dp[0];
        }
    }

    // https://leetcode.com/problems/subarray-product-less-than-k/description/
    static class Solution2 {
        public int numSubarrayProductLessThanKBinSearch(int[] nums, int k) {
            if (k == 0)
                return 0;
            var logk = Math.log(k);
            var prefix = new double[nums.length + 1];
            for (var i = 0; i < nums.length; i++) {
                prefix[i + 1] = prefix[i] + Math.log(nums[i]);
            }

            var ans = 0;
            for (var i = 0; i < prefix.length; i++) {
                int lo = i + 1, hi = prefix.length;
                while (lo < hi) {
                    var mid = lo + (hi - lo) / 2;
                    if (prefix[mid] < prefix[i] + logk - 1e-9) {
                        lo = mid + 1;
                    } else {
                        hi = mid;
                    }
                }
                ans += lo - i - 1;
            }
            return ans;
        }

        public int numSubarrayProductLessThanKSlidingWindow(int[] nums, int k) {
            var left = 0;
            var product = 1;
            var ans = 0;
            for (var right = 0; right < nums.length; right++) {
                product *= nums[right];
                while (left <= right && product >= k) {
                    product /= nums[left++];
                }
                ans += right - left + 1;
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/maximum-gap/description/
    static class Solution3 {
        public int maximumGapRadixSort(int[] nums) {
            if (nums.length < 2) {
                return 0;
            }
            var max = Integer.MIN_VALUE;
            for (int i = 0; i < nums.length; i++) {
                max = Math.max(max, nums[i]);
            }

            var bucketSort = new Object() {
                void sort(int[] arr, int pos) {
                    var buckets = new ArrayList<List<Integer>>();
                    for (int i = 0; i < 10; i++) {
                        buckets.add(i, new ArrayList<Integer>());
                    }

                    for (var num : arr) {
                        var bucketIdx = (num / pos) % 10;
                        buckets.get(bucketIdx).add(num);
                    }
                    var i = 0;
                    for (var buck : buckets) {
                        for (var num : buck) {
                            arr[i++] = num;
                        }
                    }
                }
            };

            var pos = 1;
            while ((max / pos) > 0) {
                bucketSort.sort(nums, pos);
                pos *= 10;
            }

            var ans = Integer.MIN_VALUE;
            for (int i = 0; i < nums.length - 1; i++) {
                ans = Math.max(ans, nums[i + 1] - nums[i]);
            }

            return ans;
        }
    }
}
