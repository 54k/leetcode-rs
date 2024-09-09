package leetcode_grind;

import java.util.*;

public class Day662 {
    public class ListNode {
        int val;
        ListNode next;

        ListNode() {
        }

        ListNode(int val) {
            this.val = val;
        }

        ListNode(int val, ListNode next) {
            this.val = val;
            this.next = next;
        }
    }

    // https://leetcode.com/problems/spiral-matrix-iv/description/?envType=daily-question&envId=2024-09-09
    static class Solution1 {
        public int[][] spiralMatrix(int m, int n, ListNode head) {
            int i = 0, j = 0, cur_d = 0, movement[][] = {
                    { 0, 1 },
                    { 1, 0 },
                    { 0, -1 },
                    { -1, 0 },
            };

            int[][] res = new int[m][n];
            for (int[] row : res) {
                Arrays.fill(row, -1);
            }
            while (head != null) {
                res[i][j] = head.val;
                int newi = i + movement[cur_d][0], newj = j + movement[cur_d][1];
                if (Math.min(newi, newj) < 0 || newi >= m || newj >= n || res[newi][newj] != -1) {
                    cur_d = (cur_d + 1) % 4;
                }
                i += movement[cur_d][0];
                j += movement[cur_d][1];
                head = head.next;
            }
            return res;
        }
    }

    // https://leetcode.com/problems/maximum-sum-of-subsequence-with-non-adjacent-elements/description/
    static class Solution2 {
        public int maximumSumSubsequence(int[] nums, int[][] queries) {
            var MOD = 1_000_000_007;
            var n = nums.length;
            long totalSum = 0;

            var dp_incl = new long[n]; // to track max sum till index i, when including index i
            var dp_excl = new long[n]; // to track max sum till index i, when excluding index i

            // fill dp initially
            long incll = 0, excll = 0;
            for (var i = 0; i < n; i++) {
                long newExcl = Math.max(incll, excll);
                incll = (excll + nums[i]) % MOD;
                excll = newExcl;

                dp_incl[i] = incll;
                dp_excl[i] = excll;
            }

            // now, for each query, update the nums array
            // and then start calculating incl/excl from the index that was changed
            for (var query : queries) {
                int pos = query[0];
                int val = query[1];
                nums[pos] = val;

                long incl = 0, excl = 0;
                if (pos != 0) {
                    incl = dp_incl[pos - 1];
                    excl = dp_excl[pos - 1];
                }

                for (var i = pos; i < n; i++) {
                    long newExcl = Math.max(incl, excl);
                    incl = (excl + nums[i]) % MOD;
                    excl = newExcl;

                    dp_incl[i] = incl;
                    dp_excl[i] = excl;
                }

                long maxSum = Math.max(incl, excl);
                totalSum = (totalSum + maxSum) % MOD;
            }

            return (int) totalSum;
        }
    }

    static class Solution3 {
        // each node in seg tree will have to maintain 4 values, so that we can apply
        // house robber algo
        // [0][0] : leftmost & rightmost in curr range is not selected
        // [0][1] : leftmost in curr range is not selected, rightmost is selected
        // [1][0] : leftmost in curr range is selected, rightmost is not selected
        // [1][1] : leftmost & rightmost in curr range are both selected
        public int maximumSumSubsequence(int[] nums, int[][] queries) {
            var MOD = 1_000_000_007;
            var segTree = new Node(nums, 0, nums.length - 1);
            long res = 0;

            for (var query : queries) {
                segTree.update(query[0], query[1]);
                res += segTree.getMaxSum();
                res %= MOD;
            }
            return (int) res;
        }

        static class Node {
            long inf = (long) 1e18;
            Node l = null, r = null;
            int lo, hi;
            long[][] selected = new long[2][2];

            public Node(int[] nums, int lo, int hi) {
                this.lo = lo;
                this.hi = hi;

                if (lo < hi) {
                    var mid = (lo + hi) / 2;
                    l = new Node(nums, lo, mid);
                    r = new Node(nums, mid + 1, hi);
                    combine();
                } else {
                    selected[0][0] = 0;
                    selected[0][1] = -inf;
                    selected[1][0] = -inf;
                    selected[1][1] = nums[lo]; // same as nums[hi]
                }
            }

            void combine() {
                selected[0][0] = Math.max(l.selected[0][0] + r.selected[0][0],
                        Math.max(l.selected[0][1] + r.selected[0][0], l.selected[0][0] + r.selected[1][0]));
                selected[0][1] = Math.max(l.selected[0][0] + r.selected[0][1],
                        Math.max(l.selected[0][1] + r.selected[0][1], l.selected[0][0] + r.selected[1][1]));
                selected[1][0] = Math.max(l.selected[1][0] + r.selected[0][0],
                        Math.max(l.selected[1][1] + r.selected[0][0], l.selected[1][0] + r.selected[1][0]));
                selected[1][1] = Math.max(l.selected[1][0] + r.selected[0][1],
                        Math.max(l.selected[1][1] + r.selected[0][1], l.selected[1][0] + r.selected[1][1]));
            }

            public void update(int idx, long val) {
                if (idx < lo || idx > hi)
                    return;
                if (lo == hi) {
                    selected[0][0] = 0;
                    selected[1][1] = val;
                    return;
                }
                l.update(idx, val);
                r.update(idx, val);
                combine();
            }

            long getMaxSum() {
                return Math.max(
                        selected[0][0],
                        Math.max(selected[0][1], Math.max(selected[1][0], selected[1][1])));
            }
        }
    }

    // https://leetcode.com/problems/number-of-digit-one/description/?envType=problem-list-v2&envId=dynamic-programming
    public class Solution4 {
        public int countDigitOne(int n) {
            int count = 0;
            long factor = 1; // Используем long для предотвращения переполнения при умножении.

            while (factor <= n) {
                long lower = n - (n / factor) * factor; // Младшие разряды
                long curr = (n / factor) % 10; // Текущий разряд
                long higher = n / (factor * 10); // Старшие разряды

                if (curr == 0) {
                    count += higher * factor;
                } else if (curr == 1) {
                    count += higher * factor + lower + 1;
                } else {
                    count += (higher + 1) * factor;
                }

                factor *= 10;
            }

            return count;
        }
    }
}
