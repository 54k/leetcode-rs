package leetcode_grind;

import java.util.Arrays;

public class Day1115 {
    // https://leetcode.com/problems/count-square-sum-triples/description/?envType=daily-question&envId=2025-12-08
    static class Solution1 {
        public int countTriples(int n) {
            int res = 0;
            // enumerate a and b
            for (int a = 1; a <= n; ++a) {
                for (int b = 1; b <= n; ++b) {
                    // determine if it meets the requirements
                    int c = (int) Math.sqrt(a * a + b * b + 1.0);
                    if (c <= n && c * c == a * a + b * b) {
                        ++res;
                    }
                }
            }
            return res;
        }
    }

    // https://leetcode.com/problems/minimize-product-sum-of-two-arrays/description/?envType=weekly-question&envId=2025-12-08
    static class Solution2 {
        public int minProductSum(int[] nums1, int[] nums2) {
            // Sort nums1 and nums2 in ascending order.
            Arrays.sort(nums1);
            Arrays.sort(nums2);

            // Initialize sum as 0.
            int ans = 0;
            int n = nums2.length;

            // Iterate over two sorted arrays and calculate the
            // cumulative product sum.
            for (int i = 0; i < n; ++i) {
                // Since we also sort nums2 in ascending order,
                // we need to iterate over nums2 in reverse order.
                ans += nums1[i] * nums2[n - 1 - i];
            }

            return ans;
        }
    }
}
