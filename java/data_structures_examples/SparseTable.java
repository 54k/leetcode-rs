package data_structures_examples;

class SparseTable {
    // https://leetcode.com/problems/continuous-subarrays/description/
    // https://leetcode.com/problems/continuous-subarrays/solutions/3707026/solved-using-sparse-table-binary-search-intuition-approach-c-code-with-comments/
    class Solution {
        public long continuousSubarrays1(int[] nums) {
            var n = nums.length;

            var lg = new int[n + 1];
            lg[1] = 0;
            for (int i = 2; i <= n; i++) {
                lg[i] = lg[i - 1];
                if ((1 << (lg[i] + 1)) <= i) {
                    lg[i]++;
                }
            }

            var rmq1 = new int[lg[n] + 1][n]; // min
            var rmq2 = new int[lg[n] + 1][n]; // max

            for (int i = 0; i < n; i++) {
                rmq1[0][i] = nums[i];
                rmq2[0][i] = nums[i];
            }

            for (int k = 0; k < lg[n]; k++) {
                for (int i = 0; i + (1 << k) < n; i++) {
                    rmq1[k + 1][i] = Math.min(rmq1[k][i], rmq1[k][i + (1 << k)]);
                    rmq2[k + 1][i] = Math.max(rmq2[k][i], rmq2[k][i + (1 << k)]);
                }
            }

            var rmq = new Object() {
                int min(int l, int r) {
                    int k = lg[r - l + 1];
                    return Math.min(rmq1[k][l], rmq1[k][r - (1 << k) + 1]);
                }

                int max(int l, int r) {
                    int k = lg[r - l + 1];
                    return Math.max(rmq2[k][l], rmq2[k][r - (1 << k) + 1]);
                }
            };

            long res = 0;
            int j = 0;
            for (int i = 0; i < n; i++) {
                while (true) {
                    int mx = rmq.max(j, i);
                    int mn = rmq.min(j, i);
                    if (mx - mn > 2) {
                        j++;
                    } else {
                        break;
                    }
                }

                res += i - j + 1;
            }

            return res;
        }

        public long continuousSubarrays2(int[] nums) {
            var n = nums.length;

            var lg = new int[n + 1];
            lg[1] = 0;
            for (int i = 2; i <= n; i++) {
                lg[i] = lg[i - 1];
                if ((1 << (lg[i] + 1)) <= i) {
                    lg[i]++;
                }
            }

            var rmq1 = new int[n][lg[n] + 1]; // min
            var rmq2 = new int[n][lg[n] + 1]; // max

            for (int i = 0; i < n; i++) {
                rmq1[i][0] = nums[i];
                rmq2[i][0] = nums[i];
            }

            for (int k = 1; k <= lg[n]; k++) {
                for (int i = 0; i + (1 << (k - 1)) < n; i++) {
                    rmq1[i][k] = Math.min(rmq1[i][k - 1], rmq1[i + (1 << (k - 1))][k - 1]);
                    rmq2[i][k] = Math.max(rmq2[i][k - 1], rmq2[i + (1 << (k - 1))][k - 1]);
                }
            }

            var rmq = new Object() {
                int min(int l, int r) {
                    int k = lg[r - l + 1];
                    return Math.min(rmq1[l][k], rmq1[r - (1 << k) + 1][k]);
                }

                int max(int l, int r) {
                    int k = lg[r - l + 1];
                    return Math.max(rmq2[l][k], rmq2[r - (1 << k) + 1][k]);
                }
            };

            long res = 0;
            int j = 0;
            for (int i = 0; i < n; i++) {
                while (true) {
                    int mx = rmq.max(j, i);
                    int mn = rmq.min(j, i);
                    if (mx - mn > 2) {
                        j++;
                    } else {
                        break;
                    }
                }

                res += i - j + 1;
            }

            return res;
        }
    }
}