package leetcode_grind;

public class Day368 {
    // https://leetcode.com/problems/range-sum-query-mutable/description/
    static class NumArray {
        class SegTree1 {
            int[] tree;
            int n;

            SegTree1(int[] nums) {
                n = nums.length;
                tree = new int[n * 4];
                int i = 0;
                for (int num : nums) {
                    update(1, 0, n - 1, i++, num);
                }
            }

            void update(int x, int tl, int tr, int i, int v) {
                if (tl == tr) {
                    tree[x] = v;
                    return;
                }

                int tm = (tl + tr) / 2;
                if (i <= tm) {
                    update(x * 2, tl, tm, i, v);
                } else {
                    update(x * 2 + 1, tm + 1, tr, i, v);
                }

                tree[x] = tree[x * 2] + tree[x * 2 + 1];
            }

            int sum(int k, int tl, int tr, int l, int r) {
                if (r < tl || l > tr) {
                    return 0;
                }
                if (l <= tl && tr <= r) {
                    return tree[k];
                }
                int tm = (tl + tr) / 2;
                int sl = sum(k * 2, tl, tm, l, r);
                int sr = sum(k * 2 + 1, tm + 1, tr, l, r);
                return sl + sr;
            }
        }

        class SegTree2 {
            int[] tree;
            int n;

            SegTree2(int[] nums) {
                n = nums.length;
                tree = new int[n * 2];
                int i = 0;
                for (int num : nums) {
                    update(i++, num);
                }
            }

            void update(int i, int v) {
                i += n;
                tree[i] = v;
                while (i > 0) {
                    i /= 2;
                    tree[i] = tree[i * 2] + tree[i * 2 + 1];
                }
            }

            int sum(int a, int b) {
                int sum = 0;
                a += n;
                b += n;
                while (a <= b) {
                    if (a % 2 == 1) {
                        sum += tree[a++];
                    }
                    if (b % 2 == 0) {
                        sum += tree[b--];
                    }
                    a /= 2;
                    b /= 2;
                }
                return sum;
            }
        }

        SegTree2 tree;

        public NumArray(int[] nums) {
            tree = new SegTree2(nums);
        }

        public void update(int index, int val) {
            // tree.update(1, 0, tree.n-1, index, val);
            tree.update(index, val);
        }

        public int sumRange(int left, int right) {
            // return tree.sum(1, 0, tree.n-1, left, right);
            return tree.sum(left, right);
        }
    }

    // https://leetcode.com/problems/longest-palindromic-subsequence/
    static class Solution2 {
        public int longestPalindromeSubseq(String s) {
            var dp = new int[s.length()][s.length()];
            var ans = 1;

            for (int i = s.length() - 1; i >= 0; i--) {
                dp[i][i] = 1;
                for (int j = i + 1; j < s.length(); j++) {
                    if (s.charAt(i) == s.charAt(j)) {
                        dp[i][j] = 2 + dp[i + 1][j - 1];
                    } else {
                        dp[i][j] = Math.max(dp[i + 1][j - 1], Math.max(dp[i + 1][j], dp[i][j - 1]));
                    }
                    ans = Math.max(dp[i][j], ans);
                }
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/edit-distance/
    static class Solution3 {
        public int minDistance(String word1, String word2) {
            var w1 = word1.length();
            var w2 = word2.length();

            if (w1 == 0 || w2 == 0) {
                return Math.max(w1, w2);
            }

            var dp = new int[w1 + 1][w2 + 1];

            for (int i = 1; i <= w1; i++) {
                dp[i][0] = dp[i - 1][0] + 1;

                for (int j = 1; j <= w2; j++) {
                    if (i == 1) {
                        dp[0][j] = dp[0][j - 1] + 1;
                    }

                    if (word1.charAt(i - 1) == word2.charAt(j - 1)) {
                        dp[i][j] = dp[i - 1][j - 1];
                    } else {
                        dp[i][j] = Math.min(dp[i][j - 1], Math.min(dp[i - 1][j], dp[i - 1][j - 1])) + 1;
                    }
                }
            }

            return dp[w1][w2];
        }
    }

    
}
