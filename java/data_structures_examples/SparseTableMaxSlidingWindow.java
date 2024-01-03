package data_structures_examples;

public class SparseTableMaxSlidingWindow {
    // https://leetcode.com/problems/sliding-window-maximum/description/
    static class Solution {
        public int[] maxSlidingWindow(int[] nums, int k) {
            int n = nums.length;
            int[] lg = new int[n + 1];
            lg[1] = 0;

            for (int i = 1; i <= n; i++) {
                if ((1 << (lg[i - 1] + 1)) <= i) {
                    lg[i] = lg[i - 1] + 1;
                } else {
                    lg[i] = lg[i - 1];
                }
            }

            int[][] st = new int[lg[n] + 1][n];

            for (int i = 0; i < n; i++) {
                st[0][i] = nums[i];
            }

            for (int j = 0; j < lg[n]; j++) {
                for (int i = 0; i + (1 << j) < n; i++) {
                    st[j + 1][i] = Math.max(st[j][i], st[j][i + (1 << (j))]);
                }
            }

            var mx = new Object() {
                int apply(int l, int r) {
                    var len = r - l + 1;
                    var log = lg[len];
                    var max = Math.max(st[log][l], st[log][r - (1 << log) + 1]);
                    return max;
                }
            };

            int[] result = new int[n - k + 1];

            for (int i = 0; i < n - k + 1; i++) {
                result[i] = mx.apply(i, k - 1 + i);
            }

            return result;
        }
    }
}
